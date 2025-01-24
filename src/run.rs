use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tracing::error;

pub async fn run_command(command: &str) -> Result<(), String> {
    let os = std::env::consts::OS;

    let mut child = if os == "windows" {
        Command::new("cmd")
            .arg("/c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start command")
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start command")
    };

    // handle realtime stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => error!("Error reading stdout: {}", e),
            }
        }
    }

    // handle realtime stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => eprintln!("{}", line),
                Err(e) => error!("Error reading stderr: {}", e),
            }
        }
    }

    // Wait for the command to finish
    match child.wait() {
        Ok(status) => {
            if !status.success() {
                error!("Command failed with exit code: {}", status);
                std::process::exit(status.code().unwrap_or(1));
            } else {
                Ok(()) // Add this return value for success case
            }
        }
        Err(e) => {
            error!("Failed to wait for command: {}", e);
            std::process::exit(1);
        }
    }
}
