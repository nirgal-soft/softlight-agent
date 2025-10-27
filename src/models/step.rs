use serde::{Deserialize, Serialize};
use crate::models::action::Action;
use crate::models::wait_condition::WaitCondition;

#[derive(Debug, Deserialize, Serialize)]
pub struct Step {
  pub name: String,
  pub action: Action,
  #[serde(default)]
  pub wait: Option<WaitCondition>,
  #[serde(default)]
  pub capture: bool,
  #[serde(default)]
  pub description: Option<String>,
}
