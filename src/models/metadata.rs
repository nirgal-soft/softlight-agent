use serde::{Deserialize, Serialize};
use crate::models::viewport_info::ViewportInfo;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Metadata{
  #[serde(default)]
  pub capture_elements: Vec<String>,
  #[serde(default)]
  pub notes: Option<String>,
  #[serde(default)]
  pub tags: Vec<String>,
  #[serde(default)]
  pub related_tasks: Vec<String>,
  #[serde(default)]
  pub ui_components: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PageMetadata{
  pub title: String,
  pub url: String,
  pub ready_sate: String,
  pub active_element: Option<String>,
  pub has_modals: bool,
  pub has_overlays: bool,
}

#[derive(Debug, Serialize)]
pub struct TaskMetadata{
  pub task_id: String,
  pub app: String,
  pub description: String,
  pub success: bool,
  pub execution_time_ms: u64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<String>,
  pub states: Vec<StateMetadata>,
}

#[derive(Debug, Serialize)]
pub struct StateMetadata{
  pub step_index: usize,
  pub step_name: String,
  pub filename: String,
  pub url: Option<String>,
  pub has_url: bool,
  pub viewport: ViewportInfo,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub context: Option<String>,
}
