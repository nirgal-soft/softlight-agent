use std::time::Duration;
use futures::StreamExt;
use anyhow::{Context, Result};
use chromiumoxide::{
  browser::{Browser, BrowserConfig},
  cdp::browser_protocol::{
    network::Cookie as CdpCOokie,
    emulation::SetDeviceMetricsOverrideCommandBuilder,
  },
  Page,
};

pub struct BrowserController{
  browser: Browser,
  viewport_width: u32,
  viewport_height: u32,
}

impl BrowserController{
  pub async fn new() -> Result<Self>{
    Self::with_viewport(1920, 1080).await
  }

  pub async fn with_viewport(width: u32, height: u32) -> Result<Self>{
    let(browser, mut handler) = Browser::launch(
      BrowserConfig::builder()
        .window_size(width, height)
        .disable_default_args()
        .args(vec![
          "--disable-blink-features=AutomationControlled",
          "--disable-dev-shm-usage",
          "--no-sandbox",
          "--disable-setuid-sandbox",
          "--disable-web-security",
          "--disable-features=IsolateOrigins,site-per-process",
        ])
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build browser config: {}", e))?,
    )
    .await
    .context("failed to launch browser")?;

    tokio::spawn(async move{
      while let Some(event) = handler.next().await{
        if event.is_err(){
          break;
        }
      }
    });

    Ok(Self{
      browser,
      viewport_width: width,
      viewport_height: height,
    })
  }

  pub async fn new_page(&self) -> Result<Page>{
    let page = self.browser.new_page("about:blank").await?;

    page.execute(
      SetDeviceMetricsOverrideCommandBuilder::default()
        .width(self.viewport_width)
        .height(self.viewport_height)
        .device_scale_factor(1.0)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build device metrics: {}", e))?
    )
    .await?;

    Ok(page)
  }
}
