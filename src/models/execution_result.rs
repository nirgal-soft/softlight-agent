use serde::Serialize;
use crate::models::captured_state::CapturedState;

#[derive(Debug, Serialize)]
pub struct ExecutionResult{
  pub task_id: String,
  pub app: String,
  pub description: String,
  pub success: bool,
  pub captured_states: Vec<CapturedState>,
  pub error: Option<String>,
  pub execution_time_ms: u64,
}
