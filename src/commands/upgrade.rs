use crate::run::run_command;
use semver::Version;
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
        error!(
            "Ohh no, the gitHub api request failed with status: {}",
            resp.status()
        );
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
        Some(v) => v.trim_start_matches('v'), // Remove leading 'v' if present
        None => {
            error!("Could not find tag_name in response");
            return;
        }
    };

    let current_semver = match Version::parse(current_version) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse current version: {}", e);
            return;
        }
    };

    let latest_semver = match Version::parse(latest_version) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse latest version: {}", e);
            return;
        }
    };

    if latest_semver > current_semver {
        info!("New version available: {}", latest_version);
        info!("Current version: {}", current_version);
        info!("Installing update...");

        let curl_cmd = "curl -fsSL https://raw.githubusercontent.com/lassejlv/actionfile/main/scripts/install.sh | bash";
        let _ = run_command(curl_cmd).await;
    } else {
        info!("You are already on the latest version: {}", current_version);
    }
}
