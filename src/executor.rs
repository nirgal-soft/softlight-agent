use std::time::{Duration, Instant};
use anyhow::{Context, Result};
use chromiumoxide::{
  Page,
  page::ScreenshotParams,
};
use chrono::Utc;
use serde::Serialize;
use tokio::time::sleep;
use crate::browser::{
  browser_constroller::BrowserController,
  page_extension::PageExtension,
};
use crate::models::{
  action::Action,
  captured_state::CapturedState,
  element_state::ElementState,
  execution_result::ExecutionResult,
  metadata::Metadata,
  scroll_direction::ScrollDirection,
  step::Step,
  task::Task,
  viewport_info::ViewportInfo,
  wait_condition::WaitCondition,
};
use crate::state_capture::{
  CaptureOptions,
  capture_screenshot, 
  extract_viewport_info,
  extract_page_metadata,
  capture_settled,
};

pub struct TaskExecutor{
  browser: BrowserController,
}

impl TaskExecutor{
  pub async fn new(viewport_width: u32, viewport_height: u32) -> Result<Self>{
    let browser = BrowserController::with_viewport(viewport_width, viewport_height).await?;
    Ok(Self{browser})
  }

  pub async fn execute(&self, task: Task) -> Result<ExecutionResult>{
    let start_time = Instant::now();
    let page = self.browser.new_page().await?;

    if let Some(setup) = &task.task_def.setup{
      if let Some(cookies) = &setup.cookies{
        self.browser.set_cookies(&page, cookies.clone()).await?;
      }
      if let Some(local_storage) = &setup.local_storage{
        self.browser.set_local_storage(&page, local_storage.clone()).await?;
      }
      if let Some(starting_url) = &setup.starting_url{
        let full_url = format!("{}{}", task.task_def.base_url, starting_url);
        page.goto(&full_url).await?;
      }
    }

    let mut captured_states = Vec::new();

    for (idx, step) in task.task_def.steps.iter().enumerate(){
      match self.execute_step(&page, step, &task.task_def.base_url).await{
        Ok(()) => {
          if let Some(wait) = &step.wait{
            self.wait_for_condition(&page, wait).await?;
          }

          if step.capture{
            let state = self.capture_state(&page, idx, step).await?;
            captured_states.push(state);
          }
        }
        Err(e) => {
          return Ok(ExecutionResult{
            task_id: task.task_def.id.clone(),
            app: task.task_def.app.clone(),
            description: task.task_def.description.clone(),
            success: false,
            captured_states,
            error: Some(format!("step '{}' failed: {}", step.name, e)),
            execution_time_ms: start_time.elapsed().as_millis() as u64,
          });
        }
      }
    }

    Ok(ExecutionResult{
      task_id: task.task_def.id,
      app: task.task_def.app,
      description: task.task_def.description,
      success: true,
      captured_states,
      error: None,
      execution_time_ms: start_time.elapsed().as_millis() as u64,
    })
  }

  async fn execute_step(&self, page: &Page, step: &Step, base_url: &str) -> Result<()>{
    match &step.action{
      Action::Navigate{url} => {
        let full_url = if url.starts_with("http"){
          url.clone()
        }else{
          format!("{}{}", base_url, url)
        };
        page.goto(&full_url).await?;
      }
      Action::Click{selector, wait_before_ms} => {
        if let Some(wait) = wait_before_ms{
          sleep(Duration::from_millis(*wait)).await;
        }
        let element = page.find_element(selector).await
          .with_context(|| format!("element not found: {}", selector))?;
        element.click().await?;
      }
      Action::Type{selector, value, clear_first} => {
        let element = page.find_element(selector).await
          .with_context(|| format!("input not found: {}", selector))?;

        if *clear_first{
          element.click().await?;
          #[cfg(target_os="macos")]
          element.press_key("Meta+a").await?;
          #[cfg(not(target_os="macos"))]
          element.press_key("Control+a").await?;
          element.press_key("Backspace").await?;
        }

        element.type_str(value).await?;
      }
      Action::Wait{duration_ms} => {
        sleep(Duration::from_millis(*duration_ms)).await;
      }
      Action::Scroll{direction, amount} => {
        let (x, y) = match direction{
          ScrollDirection::Down => (0, *amount),
          ScrollDirection::Up => (0, -*amount),
          ScrollDirection::Left => (-*amount, 0),
          ScrollDirection::Right => (*amount, 0),
        };
        page.evaluate(format!("window.scrollBy({}, {})", x, y)).await?;
      }
      Action::Hover{selector} => {
        let element = page.find_element(selector).await
          .with_context(|| format!("element not found: {}", selector))?;
        element.hover().await?;
      }
      Action::Press{key} => {
        page.evaluate(format!(
          "window.dispatchEvent(new KeyboardEvent('keydown', {{ key: '{}' }}))",
          key
        )).await?;
      }
      Action::Execute{script} => {
        page.evaluate(script.to_owned()).await?;
      }
    }
    Ok(())
  }

  async fn wait_for_condition(&self, page: &Page, condition: &WaitCondition) -> Result<()>{
    match condition{
      WaitCondition::Selector{value, timeout_ms, visible} => {
        if *visible{
          page.wait_for_selector_visible(value, *timeout_ms).await?;
        }else{
          page.find_element(value).await?;
        }
      }
      WaitCondition::Duration{ms} => {
        sleep(Duration::from_millis(*ms)).await;
      }
      WaitCondition::NetworkIdle{timeout_ms, ..} => {
        page.wait_for_network_idle(*timeout_ms).await?;
      }
      WaitCondition::Url{pattern, timeout_ms} => {
        let start = Instant::now();
        loop{
          if start.elapsed() > Duration::from_millis(*timeout_ms){
            anyhow::bail!("timeout waiting for url pattern: {}", pattern);
          }
          if let Some(url) = page.url().await?{
            if url.as_str().contains(pattern){
              break;
            }
          }
          sleep(Duration::from_millis(100)).await;
        }
      }
      WaitCondition::Element{selector, state,timeout_ms} => {
        let start = Instant::now();
        loop{
          if start.elapsed() > Duration::from_millis(*timeout_ms){
            anyhow::bail!("timeout waiting for element state: {}", state);
          }

          let matches = match state{
            ElementState::Visible => page.is_element_visible(selector).await?,
            ElementState::Hidden => !page.is_element_visible(selector).await?,
            ElementState::Enabled => {
              let script = format!("!document.querySelector('{}')?.disabled", selector);
              page.evaluate(script).await?.value().and_then(|v| v.as_bool()).unwrap_or(false)
            }
            ElementState::Disabled => {
              let script = format!("document.querySelector('{}')?.disabled", selector);
              page.evaluate(script).await?.value().and_then(|v| v.as_bool()).unwrap_or(false)
            }
          };

          if matches{
            break;
          }
          sleep(Duration::from_millis(100)).await;
        }
      }
    }
    Ok(())
  }

  async fn capture_state(&self, page: &Page, step_index: usize, step: &Step) -> Result<CapturedState>{
    let options = CaptureOptions::default();
    let screenshot_bytes = capture_settled(page, 300, &options).await?;
    let engine = base64::engine::general_purpose::STANDARD;
    let screenshot_base64 = base64::engine::Engine::encode(&engine, &screenshot_bytes);
    let viewport_info = extract_viewport_info(page).await?;
    let page_metadata = extract_page_metadata(page).await?;

    Ok(CapturedState{
      step_index,
      step_name: step.name.clone(),
      screenshot_base64,
      url: Some(page_metadata.url.clone()),
      has_url: !page_metadata.url.is_empty() && page_metadata.url != "about:blank",
      viewport: viewport_info,
      timestamp: Utc::now().to_rfc3339(),
      context: step.description.clone(),
      page_metadata: Some(page_metadata),
    })
  }
}
