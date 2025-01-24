use crate::run::run_command;
use tracing::{error, info};

pub async fn upgrade() {
    let current_version = env!("CARGO_PKG_VERSION");

    let api_url = "https://api.github.com/repos/lassejlv/actionfile/releases/latest";

    let client = reqwest::Client::new();
    let resp = match client
        .get(api_url)
        .header("User-Agent", "actionfile-updater")
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to fetch releases: {}", e);
            return;
        }
    };

    if !resp.status().is_success() {
        error!("GitHub API request failed with status: {}", resp.status());
        return;
    }

    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to get response text: {}", e);
            return;
        }
    };

    let json: serde_json::Value = match serde_json::from_str(&text) {
        Ok(j) => j,
        Err(e) => {
            error!("Failed to parse JSON: {}", e);
            error!("Response body: {}", text);
            return;
        }
    };

    let latest_version = match json.get("tag_name").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            error!("Could not find tag_name in response");
            return;
        }
    };

    if latest_version == current_version {
        info!("You are already on the latest version: {}", current_version);
    } else {
        info!("New version available: {}", latest_version);
        info!("Current version: {}", current_version);
        info!("Installing update...");

        let curl_cmd = "curl -fsSL https://raw.githubusercontent.com/lassejlv/actionfile/main/scripts/install.sh | bash";
        run_command(curl_cmd).await;
    }
}
