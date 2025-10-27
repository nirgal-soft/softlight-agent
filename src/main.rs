use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;

use softlight_agent::CaptureEngine;
use softlight_agent::models;
use softlight_agent::output::DatasetWriter;

#[derive(Parser)]
#[command(name="ui-capture")]
#[command(about="Capture UI states for AI agents")]
struct Cli{
  #[command(subcommand)]
  command: Commands,
}

#[derive(Parser)]
enum Commands{
  Run{
    #[arg(short, long)]
    task: PathBuf,
    #[arg(short, long, default_value = "outputs")]
    output: PathBuf,
  },

  Batch{
  #[arg(short, long)]
    tasks_dir: PathBuf,
    #[arg(short, long, default_value = "outputs")]
    output: PathBuf,
  },
}

#[tokio::main]
async fn main() -> Result<()>{
  let cli = Cli::parse();

  match cli.command{
    Commands::Run{task, output} => {
      run_single_task(&task, &output).await?;
    }
    Commands::Batch{tasks_dir, output} => {
      println!("todo...");
    }
  }

  Ok(())
}

async fn run_single_task(task_path: &PathBuf, output_dir: &PathBuf) -> Result<()>{
  println!("loading task from: {}", task_path.display());

  let yaml = tokio::fs::read_to_string(task_path).await?;
  let task: models::task::Task = serde_yaml::from_str(&yaml)?;

  println!("executing task: {} ({})", task.task_def.id, task.task_def.description);

  let executor = CaptureEngine::new();
  let result = executor.execute_task(task).await?;

  if result.success{
    println!("task completed successfully");
  }else{
    println!("task failed");
  }

  let writer = DatasetWriter::new(output_dir);
  writer.save_result(&result).await?;

  Ok(())
}

// async fn run_batch(tasks_dir: &PathBuf, output_dir: &PathBuf) -> Result<()>{
//   println!("loading tasks from: {}", tasks_dir.display());

//   let mut entires = tokio::fs::read_dir(tasks_dir).await?;
//   let mut tasks = Vec::new();

//   while let Some(entry) = entires.next_entry().await?{
//     let path = entry.path();
//     if path.extension().and_then(|s| s.to_str()) == Some("yaml")
//       || path.extension().and_then(|s| s.to_str()) == Some("yml"){
//       println!("  loading: {}", path.file_name().unwrap().to_string_lossy());
//       let yaml = tokio::fs::read_to_string(&path).await?;
//       let task: models::task::Task = serde_yaml::from_str(&yaml)?;
//       tasks.push(task);
//     }
//   }

//   let executor = CaptureEngine::new();
//   let mut results = executor.execute_batch(tasks).await?;

//   for (idx, task) in tasks.into_iter().enumerate(){
//     println!("[{}/{}] executing: {}", idx+1, results.len()+1,task.task_def.description);
//     match executor.execute_task(task).await{
//       Ok(result) => {
//         if result.success{
//           println!("  captured {} states", result.captured_states.len());
//         }else{
//           println!("  task failed");
//         }
//         results.push(result);
//       }
//       Err(e) => {
//         println!("  task failed: {}", e);
//       }
//     }
//     println!();
//   }

//   println!("saving results...");
//   let write = DatasetWriter::new(output_dir);
//   write.save_batch(results).await?;

//   Ok(())
// }
