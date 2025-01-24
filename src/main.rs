use std::process::exit;

use parser::{parse_commands, parse_npm_scripts};
use tracing::error;
use tracing_subscriber::EnvFilter;

mod commands;
mod package_detector;
mod parser;
mod run;

#[tokio::main]
async fn main() {
    // Initialize the logger with nice formatting
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .without_time()
        .init();

    let args = std::env::args().collect::<Vec<String>>();

    let mut commands = parse_commands().await;
    let npm_scripts = parse_npm_scripts().await;
    let deno_tasks = parser::parse_deno_tasks().await;

    commands.extend(npm_scripts);
    commands.extend(deno_tasks);

    if commands.is_empty() {
        error!("No commands found. Create a .actions file or package.json with scripts");
        exit(1);
    }

    let command_arg = if args.len() >= 2 {
        args[1].trim_start_matches('-').to_string()
    } else {
        String::new()
    };

    // Commands
    match command_arg.as_str() {
        "list" => {
            let _ = commands::list_commands::list_commands(commands).await;
            return;
        }
        "version" => {
            let _ = commands::version::version().await;
            return;
        }
        "upgrade" => {
            let _ = commands::upgrade::upgrade().await;
            return;
        }
        "add" => {
            let _ = commands::add::add_packages().await;
            return;
        }
        "" => {
            let _ = run::run_command(&commands[0].value).await;
            return;
        }
        _ => {
            // Look for matching command
            for command in commands {
                if command.key == command_arg {
                    let _ = run::run_command(&command.value).await;
                    return;
                }
            }
            error!("Command '{}' not found", command_arg);
            exit(1);
        }
    }
}
