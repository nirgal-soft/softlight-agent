use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::Serialize;
use crate::models::dataset_index::DatasetIndex;
use crate::models::execution_result::ExecutionResult;
use crate::models::metadata::{StateMetadata, TaskMetadata};
use crate::models::task::TaskSummary;

pub struct DatasetWriter{
  output_dir: PathBuf,
}

impl DatasetWriter{
  pub fn new<P: AsRef<Path>>(output_dir: P) -> Self{
    Self{
      output_dir: output_dir.as_ref().to_path_buf(),
    }
  }

  pub async fn save_result(&self, result: &ExecutionResult) -> Result<()>{
    let task_dir = self.output_dir
      .join(&result.app)
      .join(&result.task_id);

    tokio::fs::create_dir_all(&task_dir)
      .await
      .context("failed to create task directory")?;

    let states: Vec<StateMetadata> = result.captured_states.iter().enumerate().map(|(idx, state)|{
      StateMetadata{
        step_index: state.step_index,
        step_name: state.step_name.clone(),
        filename: format!("{:02}-{}.png", idx+1, slugify(&state.step_name)),
        url: state.url.clone(),
        has_url: state.has_url,
        viewport: state.viewport.clone(),
        context: state.context.clone(),
      }
    }).collect();

    let metadata = TaskMetadata{
      task_id: result.task_id.clone(),
      app: result.app.clone(),
      description: result.description.clone(),
      success: result.success,
      execution_time_ms: result.execution_time_ms,
      error: result.error.clone(),
      states,
    };

    let md_json = serde_json::to_string_pretty(&metadata)?;
    tokio::fs::write(task_dir.join("metadata.json"), md_json)
      .await
      .context("failed to write metadata")?;

    for(idx, state) in result.captured_states.iter().enumerate(){
      let filename = format!("{:02}-{}.png", idx+1, slugify(&state.step_name));
      let image_path = task_dir.join(&filename);

      let engine = base64::engine::general_purpose::STANDARD;
      let image_bytes = base64::engine::Engine::decode(&engine, state.screenshot_base64.as_bytes())?;
      tokio::fs::write(&image_path, &image_bytes)
        .await
        .with_context(|| format!("failed to write screenshot: {}", filename))?;
    }

    println!("saved {} states to {}", result.captured_states.len(), task_dir.display());
    Ok(())
  }

  pub async fn save_batch(&self, results: Vec<ExecutionResult>) -> Result<()>{
    for result in &results{
      self.save_result(result).await?;
    }

    self.generate_index(&results).await?;
    Ok(())
  }

  async fn generate_index(&self, results: &[ExecutionResult]) -> Result<()>{
    let index = DatasetIndex{
      generated_at: chrono::Utc::now().to_rfc3339(),
      total_tasks: results.len(),
      successful_tasks: results.iter().filter(|r| r.success).count(),
      total_states: results.iter().map(|r| r.captured_states.len()).sum(),
      tasks: results.iter().map(|r| TaskSummary{
        task_id: r.task_id.clone(),
        app: r.app.clone(),
        description: r.description.clone(),
        success: r.success,
        state_count: r.captured_states.len(),
        path: format!("{}/{}", r.app, r.task_id),
      }).collect(),
    };

    let index_json = serde_json::to_string_pretty(&index)?;
    tokio::fs::write(self.output_dir.join("index.json"), index_json)
      .await
      .context("failed to write index")?;

    println!("\ngenerated dataset index: {}/index.json", self.output_dir.display());
    Ok(())
  }
}

fn slugify(s: &str) -> String{
  s.to_lowercase()
    .chars()
    .map(|c| {
      if c.is_alphanumeric(){
        c
      }else if c.is_whitespace() || c == '-'{
        '-'
      }else{
        '_'
      }
    })
    .collect::<String>()
    .split('-')
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join("-")
}
