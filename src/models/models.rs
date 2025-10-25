use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Action{
  Navigate{url: String},
  Click{selector: String},
  Type{selector: String, value: String},
  Wait{duration_ms: u64},
  Scroll{direction: String, amount: i32},
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum WaitCondition{
  Seelctor{value: String, timeout_ms: u64},
  Duration{ms: u64},
  NetworkIdle{timeout_ms: u64},
}

#[derive(Debug, Deserialize)]
struct Step{
  name: String,
  action: String,
  wait: Option<WaitCondition>,
  capture: bool,
}

#[derive(Debug, Deserialize)]
struct Task{
  id: String,
  app: String,
  description: String,
  base_url: String,
  setup: Option<Setup>,
  steps: Vec<Step>,
  metadata: Option<Metadata>,
}
