use serde::Deserialize;
use crate::models::element_state::ElementState;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WaitCondition{
  Selector{
    value: String,
    #[serde(default = "default_timeout")]
    timeout_ms: u64,
    #[serde(default)]
    visible: bool,
  },
  Duration{ms: u64},
  NetworkIdle{
    #[serde(default = "default_timeout")]
    timeout_ms: u64,
    #[serde(default = "default_max_connections")]
    max_connections: u8,
  },
  Url{
    pattern: String,
    #[serde(default = "default_timeout")]
    timeout_ms: u64,
  },
  Element{
    selector: String,
    state: ElementState,
    #[serde(default = "default_timeout")]
    timeout_ms: u64,
  }
}

fn default_timeout() -> u64 {5000}
fn default_max_connections() -> u8 {2}
