use crate::parser;

pub async fn list_commands(commands: Vec<parser::Command>) {
    println!("Available commands:\n");

    for command in commands {
        println!("{} = {}", command.key, command.value);
    }
}
