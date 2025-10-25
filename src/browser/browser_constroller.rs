use std::time::Duration;
use std::collections::HashMap;
use futures::StreamExt;
use anyhow::{Context, Result};
use chromiumoxide::{
  browser::{Browser, BrowserConfig},
  cdp::browser_protocol::{
    network::{
      Cookie as CdpCookie,
      SetCookieParams,
    },
    emulation::SetDeviceMetricsOverrideParamsBuilder,
  },
  Page,
};
use tokio::time::sleep;
use crate::models::cookie::Cookie;

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
      loop{
        if handler.next().await.is_none(){
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
      SetDeviceMetricsOverrideParamsBuilder::default()
        .width(self.viewport_width)
        .height(self.viewport_height)
        .device_scale_factor(1.0)
        .mobile(false)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build device metrics: {}", e))?
    )
    .await?;

    Ok(page)
  }

  pub async fn set_cookies(&self, page: &Page, cookies: Vec<Cookie>) -> Result<()>{
    for cookie in cookies{
      let set_cookie_params = SetCookieParams::builder()
        .name(cookie.name)
        .value(cookie.value)
        .domain(cookie.domain)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build set cookie params: {}", e))?;

      page.execute(set_cookie_params)
        .await
        .context("failed to set cookie")?;
    }
    Ok(())
  }

  pub async fn set_local_storage(&self, page: &Page, items: HashMap<String, String>) -> Result<()>{
    for (key, value) in items{
      let script = format!(
        "localStorage.setItem('{}', '{}');",
        key.replace('\'', "\\'"),
        value.replace('\'', "\\'")
      );
      page.evaluate(script).await?;
    }
    Ok(())
  }

  pub async fn close(mut self) -> Result<()>{
    self.browser.close().await?;
    sleep(Duration::from_millis(500)).await;
    Ok(())
  }
}
