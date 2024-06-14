use std::{collections::HashMap, fs, io, process::Command};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Task {
    pub command: String,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct Tasks {
    pub run: Option<Task>,
    pub build: Option<Task>,
}

fn parser() -> Result<String, &'static str> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => Ok(args[1].clone()),
        3 => Ok(args[2].clone()),
        _ => Err("Invalid number of arguments"),
    }
}

fn read_helix_json() -> Result<Tasks, io::Error> {
    let file_content = fs::read_to_string(".helix/helix.json")?;
    let tasks: Tasks = serde_json::from_str(&file_content)?;
    Ok(tasks)
}

fn runner(task: Option<Task>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(task) = task {
        let mut command = Command::new(&task.command);
        if let Some(args) = task.args {
            command.args(args);
        }
        if let Some(envs) = task.env {
            for (key, value) in envs {
                command.env(key, value);
            }
        }
        let output = command.output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

fn launcher() -> Result<(), Box<dyn std::error::Error>> {
    match parser() {
        Ok(cmd) => {
            let tasks = read_helix_json()?;
            match cmd.as_str() {
                "run" => runner(tasks.run)?,
                "build" => runner(tasks.build)?,
                _ => return Err("Unknown command".into()),
            }
        }
        Err(e) => return Err(e.into()),
    }
    Ok(())
}

fn main() {
    if let Err(e) = launcher() {
        eprintln!("Error: {e}");
    }
}
