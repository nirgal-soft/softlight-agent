use std::time::Duration;
use anyhow::{anyhow, Result};
use chromiumoxide::{
  Page,
  page::ScreenshotParams,
  cdp::browser_protocol::page::CaptureScreenshotFormat,
};
use serde::Serialize;
use tokio::time::sleep;
use crate::models::{
  metadata::PageMetadata,
  viewport_info::ViewportInfo
};

#[derive(Debug, Serialize)]
pub struct CaptureOptions{
  pub full_page: bool,
  pub omit_background: bool,
}

impl Default for CaptureOptions{
  fn default() -> Self{
    Self{
      full_page: false,
      omit_background: false,
    }
  }
}

pub async fn capture_screenshot(page: &Page, options: &CaptureOptions) -> Result<Vec<u8>>{
  let mut params = ScreenshotParams::builder();

  params = params.full_page(options.full_page);
  params = params.omit_background(options.omit_background);

  params = params.format(CaptureScreenshotFormat::Png);
  page.screenshot(params.build()).await.map_err(|e| anyhow!("screenshot capture failed: {}", e))
}

pub async fn extract_viewport_info(page: &Page) -> Result<ViewportInfo>{
  let viewport_data: serde_json::Value = page
    .evaluate(
      r#"
      ({
        width: window.innerWidth,
        height: window.innerHeight,
        scrollY: window.scrollY,
      })
      "#
    )
    .await?
    .into_value()?;

  Ok(ViewportInfo{
    width: viewport_data["width"].as_u64().unwrap() as u32,
    height: viewport_data["height"].as_u64().unwrap() as u32,
    scroll_y: viewport_data["scrollY"].as_u64().unwrap() as u32,
  })
}

pub async fn extract_page_metadata(page: &Page) -> Result<PageMetadata>{
  let metadata: serde_json::Value = page
    .evaluate(
      r#"
      ({
        title: document.title,
        url: document.location.href,
        readyState: document.readyState,
        activeElement: document.activeElement,
        hasModals: document.querySelectorAll('[role="dialog"]').length > 0,
        hasOverlays: document.querySelectorAll('[class*="overlay"]').length > 0,
      })
      "#
    )
    .await?
    .into_value()?;

  Ok(PageMetadata{
    title: metadata["title"].as_str().unwrap_or("").to_string(),
    url: metadata["url"].as_str().unwrap_or("").to_string(),
    ready_sate: metadata["readyState"].as_str().unwrap_or("").to_string(),
    active_element: metadata["activeElement"].as_str().map(String::from),
    has_modals: metadata["hasModals"].as_bool().unwrap_or(false),
    has_overlays: metadata["hasOverlays"].as_bool().unwrap_or(false),
  })
}

pub async fn wait_for_settle(page: &Page, duration_ms: u64) -> Result<()>{
  page.evaluate(
    r#"
    new Promise(resolve => {
      requestAnimationFrame(() => {
        requestAnimationFrame(resolve);
      });
    })
    "#
  ).await?;

  sleep(Duration::from_millis(duration_ms)).await;

  Ok(())
}

pub async fn capture_settled(page: &Page, settle_ms: u64, options: &CaptureOptions) -> Result<Vec<u8>>{
  wait_for_settle(page, settle_ms).await?;
  capture_screenshot(page, options).await
}
