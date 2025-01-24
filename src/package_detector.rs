use std::io::Error;
use std::path::Path;

pub enum PackageManager {
    Bun,
    Npm,
    Pnpm,
    Yarn,
    Go,
    Pip,
    Cargo,
}

pub async fn detect_package_manager() -> Result<PackageManager, Error> {
    if Path::new("package-lock.json").exists() {
        Ok(PackageManager::Npm)
    } else if Path::new("bun.lock").exists() {
        Ok(PackageManager::Bun)
    } else if Path::new("pnpm-lock.yaml").exists() {
        Ok(PackageManager::Pnpm)
    } else if Path::new("yarn.lock").exists() {
        Ok(PackageManager::Yarn)
    } else if Path::new("go.mod").exists() {
        Ok(PackageManager::Go)
    } else if Path::new("Cargo.toml").exists() {
        Ok(PackageManager::Cargo)
    } else if Path::new("requirements.txt").exists() {
        Ok(PackageManager::Pip)
    } else {
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "No package manager found",
        ))
    }
}

pub async fn return_install_cmd() -> Result<String, Error> {
    let package_manager = match detect_package_manager().await {
        Ok(pm) => pm,
        Err(e) => return Err(e),
    };

    match package_manager {
        PackageManager::Npm => Ok("npm install".to_string()),
        PackageManager::Bun => Ok("bun install".to_string()),
        PackageManager::Pnpm => Ok("pnpm install".to_string()),
        PackageManager::Yarn => Ok("yarn install".to_string()),
        PackageManager::Go => Ok("go install".to_string()),
        PackageManager::Cargo => Ok("cargo install".to_string()),
        PackageManager::Pip => Ok("pip install".to_string()),
    }
}
