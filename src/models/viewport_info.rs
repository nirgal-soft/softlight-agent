use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ViewportInfo{
  pub width: u32,
  pub height: u32,
  pub scroll_y: u32,
}
