use tracing::{error, info};

use crate::{package_detector::return_install_cmd, run::run_command};

pub async fn install() {
    let install_cmd = match return_install_cmd().await {
        Ok(cmd) => cmd,
        Err(e) => {
            error!("Failed to get install command: {}", e);
            return;
        }
    };

    let _ = match run_command(&install_cmd).await {
        Ok(_) => {
            info!("Packages are installed!")
        }
        Err(e) => {
            error!("Failed to run install command: {}", e);
            return;
        }
    };
}
