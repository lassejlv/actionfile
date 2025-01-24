use tracing::{error, info};

use crate::{package_detector::return_install_cmd, run::run_command};

pub async fn add_packages() {
    let packages_args = std::env::args().skip(2).collect::<Vec<String>>();

    let install_cmd = match return_install_cmd().await {
        Ok(cmd) => cmd,
        Err(e) => {
            error!("Failed to get install command: {}", e);
            return;
        }
    };

    let cmd = format!("{} {}", install_cmd, packages_args.join(" "));

    let _ = match run_command(&cmd).await {
        Ok(_) => {
            info!("Packages are installed!")
        }
        Err(e) => {
            error!("Failed to run install command: {}", e);
            return;
        }
    };
}
