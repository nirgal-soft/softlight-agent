use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ElementState{
  Visible,
  Hidden,
  Enabled,
  Disabled,
}
