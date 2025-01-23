pub struct Command {
    pub key: String,
    pub value: String,
}

pub async fn parse_commands() -> Vec<Command> {
    let file_name = ".actions";
    let file_exist = tokio::fs::metadata(file_name).await.is_ok();
    let example_content = "hello = echo 'hello'";

    if !file_exist {
        let _ = tokio::fs::write(file_name, example_content).await;
        println!("Created file {file_name} with an example command {example_content}");
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
