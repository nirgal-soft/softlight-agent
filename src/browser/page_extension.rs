use std::time::{Duration, Instant};
use anyhow::Result;
use chromiumoxide::Page;
use tokio::time::sleep;

#[async_trait::async_trait]
pub trait PageExtension{
  async fn wait_for_selector_visible(&self, selector: &str, timeout_ms: u64) -> Result<()>;
  async fn wait_for_network_idle(&self, timeout_ms: u64) -> Result<()>;
  async fn is_element_visible(&self, selector: &str) -> Result<bool>;
}

#[async_trait::async_trait]
impl PageExtension for Page{
  async fn wait_for_selector_visible(&self, selector: &str, timeout_ms: u64) -> Result<()>{
    let timeout = Duration::from_millis(timeout_ms);
    let start = Instant::now();

    loop{
      if start.elapsed() > timeout{
        anyhow::bail!("timeout waiting for selector: {}", selector);
      }

      let script = format!(
        r#"
        (() => {{
          const el = document.querySelector('{}');
          if(!el) return false;
          const rect = el.getBoundingClientRect();
          const style = window.getComputedStyle(el);
          return rect.height > 0 &&
                 rect.width > 0 &&
                 style.visibility !== 'hidden' &&
                 style.display !== 'none';
        }})()
        "#,
        selector.replace('\'', "\\'")
      );

      match self.evaluate(script).await{
        Ok(result) => {
          if let Some(visible) = result.value(){
            if let Some(true) = visible.as_bool(){
              return Ok(());
            }
          }
        }
        Err(_) => {
          //element not found, continue waiting
        }
      }

      sleep(Duration::from_millis(100)).await;
    }
  }

  async fn wait_for_network_idle(&self, timeout_ms: u64) -> Result<()>{
    sleep(Duration::from_millis(timeout_ms)).await;
    Ok(())
  }

  async fn is_element_visible(&self, selector: &str) -> Result<bool>{
    let script = format!(
      r#"
      (() => {{
        const el = document.querySelector('{}');
        if(!el) return false;
        const rect = el.getBoundingClientRect();
        const style = window.getComputedStyle(el);
        return rect.height > 0 &&
               rect.width > 0 &&
               style.visibility !== 'hidden' &&
               style.display !== 'none';
      }})()
      "#,
      selector.replace('\'', "\\'")
    );

    match self.evaluate(script).await{
      Ok(result) => {
        if let Some(value) = result.value(){
          Ok(value.as_bool().unwrap_or(false))
        }else{
          Ok(false)
        }
      }
      Err(_) => Ok(false)
    }
  }
}
