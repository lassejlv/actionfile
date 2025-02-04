pub async fn version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("{}", version);
}
