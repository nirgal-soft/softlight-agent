use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::captured_state::CapturedState;
use crate::models::metadata::Metadata;
use crate::models::setup::Setup;
use crate::models::step::Step;

#[derive(Debug, Deserialize)]
pub struct Task{
  #[serde(flatten)]
  pub task_def: TaskDefinition,
  pub metadata: Option<Metadata>,
}

#[derive(Debug, Deserialize)]
pub struct TaskDefinition{
  pub id: String,
  pub app: String,
  pub description: String,
  pub base_url: String,
  pub setup: Option<Setup>,
  pub steps: Vec<Step>,
}

#[derive(Debug, Serialize)]
pub struct TaskOutput{
  pub task_id: String,
  pub app: String,
  pub description: String,
  pub captured_at: String,
  pub states: Vec<CapturedState>,
  pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize)]
pub struct TaskSummary{
  pub task_id: String,
  pub app: String,
  pub description: String,
  pub success: bool,
  pub state_count: usize,
  pub path: String,
}
