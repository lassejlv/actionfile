// A function that takes a JSON string and returns a Result with a JSON object
pub async fn parse_json(json: &str) -> Result<serde_json::Value, String> {
    match serde_json::from_str(json) {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("Failed to parse JSON: {}", e)),
    }
}
