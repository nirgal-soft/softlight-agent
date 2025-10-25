use serde::{Deserialize, Serialize};

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
