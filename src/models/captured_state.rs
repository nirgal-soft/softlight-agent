use serde::Serialize;
use crate::models::viewport_info::ViewportInfo;

#[derive(Debug, Serialize)]
pub struct CapturedState{
  pub step_index: usize,
  pub step_name: String,
  pub filename: String,
  pub url: Option<String>,
  pub viewport: ViewportInfo,
  pub timestamp: String,
  pub context: Option<String>,
}
