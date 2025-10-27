use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementState{
  Visible,
  Hidden,
  Enabled,
  Disabled,
}

impl fmt::Display for ElementState{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
    match self{
      ElementState::Visible => write!(f, "visible"),
      ElementState::Hidden => write!(f, "hidden"),
      ElementState::Enabled => write!(f, "enabled"),
      ElementState::Disabled => write!(f, "disabled"),
    }
  }
}
