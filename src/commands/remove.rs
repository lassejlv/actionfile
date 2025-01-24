use tracing::{error, info, warn};

use crate::{package_detector::return_remove_cmd, run::run_command};

pub async fn remove_packages() {
    let packages_args = std::env::args().skip(2).collect::<Vec<String>>();

    if packages_args.len() == 0 {
        warn!("No packages to remove");
        return;
    }

    let install_cmd = match return_remove_cmd().await {
        Ok(cmd) => cmd,
        Err(e) => {
            error!("Failed to get remove command: {}", e);
            return;
        }
    };

    let cmd = format!("{} {}", install_cmd, packages_args.join(" "));

    let _ = match run_command(&cmd).await {
        Ok(_) => {
            info!("Packages are removed!")
        }
        Err(e) => {
            error!("Failed to run remove command: {}", e);
            return;
        }
    };
}
