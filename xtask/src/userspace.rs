use anyhow::{Context, Result, anyhow};
use std::process::Command;


pub fn build_userspace() -> Result<()> {
    println!("⚙️ Compiling user-space targets...");

    let status = Command::new("cargo")
        .args(["build", "--package", "monad-firewall-core"])
        .status()
        .context("Failed to execute cargo build for user-space core")?;

    if !status.success() {
        return Err(anyhow!("User-space compilation failed"));
    }

    Ok(())
}

