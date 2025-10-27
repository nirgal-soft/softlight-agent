use anyhow::Result;

mod browser;
mod executor;
mod state_capture;
pub mod models;
pub mod output;
pub mod task_definition;

use crate::executor::TaskExecutor;
use crate::models::execution_result::ExecutionResult;
use crate::models::task::Task;

pub struct CaptureEngine{
  viewport_width: u32,
  viewport_height: u32,
}

impl Default for CaptureEngine{
  fn default() -> Self{
    Self::new()
  }
}

impl CaptureEngine{
  pub fn new() -> Self{
    Self{
      viewport_width: 1920,
      viewport_height: 1080,
    }
  }

  pub fn with_viewport(width: u32, height: u32) -> Self{
    Self{
      viewport_width: width,
      viewport_height: height,
    }
  }

  pub async fn execute_task(&self, task: Task) -> Result<ExecutionResult>{
    let executor = TaskExecutor::new(self.viewport_width, self.viewport_height).await?;
    executor.execute(task).await
  }

  pub async fn execute_batch(&self, tasks: Vec<Task>) -> Result<Vec<ExecutionResult>>{
    let executor = TaskExecutor::new(self.viewport_width, self.viewport_height).await?;

    let mut results = Vec::new();
    for task in tasks{
      match executor.execute(task).await{
        Ok(result) => results.push(result),
        Err(e) =>{
          eprintln!("error executing task: {}", e);
        }
      }
    }

    Ok(results)
  }

  pub fn load_task_from_yaml(yaml: &str) -> Result<Task>{
    serde_yaml::from_str(yaml)
      .map_err(|e| anyhow::anyhow!("failed to parse task definition: {}", e))
  }

  pub async fn load_task_from_file(path: &std::path::Path) -> Result<Task>{
    let yaml = tokio::fs::read_to_string(path).await?;
    Self::load_task_from_yaml(&yaml)
  }
}
