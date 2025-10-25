use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct PageMetadata{
  pub title: String,
  pub url: String,
  pub ready_sate: String,
  pub active_element: Option<String>,
  pub has_modals: bool,
  pub has_overlays: bool,
}
