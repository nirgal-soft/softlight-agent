use serde::Serialize;
use crate::models::{
  metadata::PageMetadata,
  viewport_info::ViewportInfo
};

#[derive(Debug, Serialize)]
pub struct CapturedState{
  pub step_index: usize,
  pub step_name: String,
  pub screenshot_base64: String,
  pub url: Option<String>,
  pub has_url: bool,
  pub viewport: ViewportInfo,
  pub timestamp: String,
  pub context: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page_metadata: Option<PageMetadata>,
}
