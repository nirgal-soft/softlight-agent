use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrollDirection{
  Up,
  Down,
  Left,
  Right,
}
