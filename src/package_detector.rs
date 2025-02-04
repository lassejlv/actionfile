use std::io::Error;

use crate::helpers::file_exists;

pub enum PackageManager {
    Bun,
    Npm,
    Pnpm,
    Deno,
    Yarn,
    Go,
    Pip,
    Cargo,
}

pub async fn detect_package_manager() -> Result<PackageManager, Error> {
    match () {
        _ if file_exists("package-lock.json") => Ok(PackageManager::Npm),
        _ if file_exists("bun.lock") || file_exists("bun.lockb") => Ok(PackageManager::Bun),
        _ if file_exists("pnpm-lock.yaml") => Ok(PackageManager::Pnpm),
        _ if file_exists("deno.json") || file_exists("deno.lock") => Ok(PackageManager::Deno),
        _ if file_exists("yarn.lock") => Ok(PackageManager::Yarn),
        _ if file_exists("go.mod") => Ok(PackageManager::Go),
        _ if file_exists("Cargo.toml") => Ok(PackageManager::Cargo),
        _ if file_exists("requirements.txt") => Ok(PackageManager::Pip),
        _ => Err(Error::new(
            std::io::ErrorKind::NotFound,
            "No package manager found",
        )),
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
        PackageManager::Deno => Ok("deno add".to_string()),
        PackageManager::Pnpm => Ok("pnpm install".to_string()),
        PackageManager::Yarn => Ok("yarn install".to_string()),
        PackageManager::Go => Ok("go install".to_string()),
        PackageManager::Cargo => Ok("cargo add".to_string()),
        PackageManager::Pip => Ok("pip install".to_string()),
    }
}

pub async fn return_remove_cmd() -> Result<String, Error> {
    let package_manager = match detect_package_manager().await {
        Ok(pm) => pm,
        Err(e) => return Err(e),
    };

    match package_manager {
        PackageManager::Npm => Ok("npm uninstall".to_string()),
        PackageManager::Bun => Ok("bun rm".to_string()),
        PackageManager::Deno => Ok(not_supported("Deno").to_string()),
        PackageManager::Pnpm => Ok("pnpm rm".to_string()),
        PackageManager::Yarn => Ok("yarn remove".to_string()),
        PackageManager::Go => Ok(not_supported("Go").to_string()),
        PackageManager::Cargo => Ok(not_supported("Cargo").to_string()),
        PackageManager::Pip => Ok("pip uninstall".to_string()),
    }
}

fn not_supported(name: &str) -> String {
    format!("echo \"{} is not supported\"", name)
}
