use serde::{Deserialize, Serialize};
use std::process::exit;

use tracing::{error, info, warn};

use crate::{helpers::confirm, parser_json::parse_json};

pub struct Command {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
struct PackageJson {
    scripts: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
struct DenoTasks {
    tasks: Option<serde_json::Map<String, serde_json::Value>>,
}

pub async fn parse_commands() -> Vec<Command> {
    let file_name = ".actions";
    let file_exist = tokio::fs::metadata(file_name).await.is_ok();
    let example_content = "hello = echo 'hello'";

    let npm_exist = tokio::fs::metadata("package.json").await.is_ok();
    let deno_exist = tokio::fs::metadata("deno.json").await.is_ok();

    if !file_exist && !npm_exist && !deno_exist {
        let confirmed = confirm("Create a .actions example file? (y/n)");

        if !confirmed {
            return Vec::new();
        }

        let _ = tokio::fs::write(file_name, example_content).await;
        info!("Created file {file_name} with an example command '{example_content}'");
    }

    let file_content = match tokio::fs::read_to_string(file_name).await {
        Ok(content) => content,
        Err(e) => {
            if !npm_exist && !deno_exist {
                error!("Failed to read {file_name}: {e}");
                return Vec::new();
            }

            return Vec::new();
        }
    };

    let mut commands = Vec::new();

    // Loop and split the file content into lines
    for (line_number, line) in file_content.lines().enumerate() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Skip comments
        if line.trim().starts_with('#') {
            continue;
        }

        match line.split_once('=') {
            Some((key, value)) => {
                let trimmed_key = key.trim();
                let trimmed_value = value.trim();

                if !trimmed_key.is_empty() && !trimmed_value.is_empty() {
                    commands.push(Command {
                        key: trimmed_key.to_string(),
                        value: trimmed_value.to_string(),
                    });
                }
            }
            None => {
                warn!(
                    "Invalid format in {} at line {}: {}",
                    file_name,
                    line_number + 1,
                    line
                );
                continue;
            }
        }
    }

    commands
}

pub async fn parse_npm_scripts() -> Vec<Command> {
    let file_name = "package.json";
    let file_exist = tokio::fs::metadata(file_name).await.is_ok();

    let mut commands = Vec::new();

    if !file_exist {
        return commands;
    }

    let file_content = tokio::fs::read_to_string(file_name).await.unwrap();

    let package_json: PackageJson = match parse_json(&file_content).await {
        Ok(json) => serde_json::from_value(json).unwrap_or_else(|e| {
            warn!("Failed to parse {file_name}: {}", e);
            PackageJson { scripts: None }
        }),
        Err(e) => {
            warn!("Failed to parse {file_name}: {}", e);
            return commands;
        }
    };

    if let Some(scripts) = package_json.scripts {
        for (name, command) in scripts {
            if let Some(cmd) = command.as_str() {
                commands.push(Command {
                    key: name,
                    value: cmd.to_string(),
                });
            }
        }
    }

    if commands.is_empty() {
        exit(1);
    }

    commands
}

pub async fn parse_deno_tasks() -> Vec<Command> {
    let file_name = "deno.json";
    let file_exist = tokio::fs::metadata(file_name).await.is_ok();

    let mut commands = Vec::new();

    if !file_exist {
        return commands;
    }

    let file_content = tokio::fs::read_to_string(file_name).await.unwrap();

    let deno_tasks: DenoTasks = match parse_json(&file_content).await {
        Ok(json) => serde_json::from_value(json).unwrap_or_else(|e| {
            warn!("Failed to parse {file_name}: {}", e);
            DenoTasks { tasks: None }
        }),
        Err(e) => {
            warn!("Failed to parse {file_name}: {}", e);
            return commands;
        }
    };

    if let Some(tasks) = deno_tasks.tasks {
        for (name, command) in tasks {
            if let Some(cmd) = command.as_str() {
                commands.push(Command {
                    key: name,
                    value: cmd.to_string(),
                });
            }
        }
    }

    if commands.is_empty() {
        exit(1);
    }

    commands
}
