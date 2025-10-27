use serde::Serialize;
use crate::models::task::TaskSummary;

#[derive(Debug, Serialize)]
pub struct DatasetIndex{
  pub generated_at: String,
  pub total_tasks: usize,
  pub successful_tasks: usize,
  pub total_states: usize,
  pub tasks: Vec<TaskSummary>,
}
