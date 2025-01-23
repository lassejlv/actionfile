pub async fn run_command(command: &str) {
    let os = std::env::consts::OS;

    let output: std::process::Output;

    if os == "windows" {
        output = std::process::Command::new("cmd")
            .arg("/c")
            .arg(command)
            .output()
            .expect("Failed to execute command");
    } else {
        output = std::process::Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
