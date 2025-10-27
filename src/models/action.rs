use serde::{Deserialize, Serialize};
use crate::models::scroll_direction::ScrollDirection;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action{
  Navigate{url: String},
  Click{
    selector: String,
    #[serde(default)]
    wait_before_ms: Option<u64>,
  },
  Type{
    selector: String,
    value: String,
    #[serde(default = "default_clear")]
    clear_first: bool,
  },
  Wait{duration_ms: u64},
  Scroll{
    direction: ScrollDirection,
    amount: i32,
  },
  Hover{selector: String},
  Press{key: String},
  Execute{script: String},
}

fn default_clear() -> bool{true}
