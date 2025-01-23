use parser::parse_commands;

mod commands;
mod parser;
mod run;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let commands = parse_commands().await;

    // Run the first command in commands if no command is provided
    if args.len() == 1 {
        let _ = run::run_command(&commands[0].value).await;
        return;
    }

    // Commands
    if args[1] == "--list" {
        let _ = commands::list_commands::list_commands(commands).await;
        return;
    } else if args[1] == "--version" {
        let _ = commands::version::version().await;
        return;
    }

    for command in commands {
        let is_a_match = command.key == args[1];

        if is_a_match {
            let _ = run::run_command(&command.value).await;
            return;
        } else {
            continue;
        }
    }

    println!("Command '{}' not found", args[1]);
}
