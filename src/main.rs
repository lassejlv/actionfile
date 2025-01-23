use parser::parse_commands;

mod parser;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let commands = parse_commands().await;

    for command in commands {
        let is_a_match = command.key == args[1];

        if is_a_match {
            // Run the the command command.value (already trimmed etc)
            let output = std::process::Command::new("bash")
                .arg("-c")
                .arg(command.value)
                .output()
                .expect("Failed to execute command");

            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));

            return;
        } else {
            continue;
        }
    }

    println!("Command '{}' not found", args[1]);
}
