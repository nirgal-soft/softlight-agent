use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ViewportInfo{
  pub width: u32,
  pub hiehg: u32,
  pub scroll_y: i32,
}
