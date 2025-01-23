use serde::{Deserialize, Serialize};
use serde_json;
use std::process::exit;

use tracing::{info, warn};

pub struct Command {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
struct PackageJson {
    scripts: Option<serde_json::Map<String, serde_json::Value>>,
}

pub async fn parse_commands() -> Vec<Command> {
    let file_name = ".actions";
    let file_exist = tokio::fs::metadata(file_name).await.is_ok();
    let example_content = "hello = echo 'hello'";

    if !file_exist {
        let _ = tokio::fs::write(file_name, example_content).await;
        info!("Created file {file_name} with an example command '{example_content}'");
    }

    let file_content = tokio::fs::read_to_string(file_name).await.unwrap();

    let mut commands = Vec::new();

    // Loop and split the file content into lines
    for line in file_content.lines() {
        let (key, value) = line.split_once('=').unwrap();

        let trimmed_key = key.trim();
        let trimmed_value = value.trim();

        commands.push(Command {
            key: trimmed_key.to_string(),
            value: trimmed_value.to_string(),
        });
    }

    commands
}

pub async fn parse_npm_scripts() -> Vec<Command> {
    let file_name = "package.json";
    let file_exist = tokio::fs::metadata(file_name).await.is_ok();

    let mut commands = Vec::new();

    if !file_exist {
        warn!("File {file_name} does not exist");
        return commands;
    }

    let file_content = tokio::fs::read_to_string(file_name).await.unwrap();

    let package_json: PackageJson = match serde_json::from_str(&file_content) {
        Ok(json) => json,
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
        warn!("No npm scripts found in {file_name}");
        exit(1);
    }

    info!("Found {} npm scripts in {file_name}", commands.len());

    commands
}
