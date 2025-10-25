use anyhow::Result;

pub mod browser;
pub mod executor;
pub mod models;
pub mod output;
pub mod state_capture;
pub mod task_definition;

pub struct CaptureEngine{
  viewport_width: u32,
  viewport_height: u32,
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

  pub async fn execute_task(&self, task: task_definition::Task) -> Result<()>{
    let executor = 
  }
}
