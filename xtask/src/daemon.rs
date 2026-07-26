use anyhow::{Context, Result, anyhow};
use std::process::Command;
use which::which;

pub fn run_daemon(interface: &str) -> Result<()> {
    println!(
        "🚀 Executing monad-firewall-core on interface: {}...",
        interface
    );

    // Check if sudo is present to handle capability escalation
    let has_sudo = which("sudo").is_ok();

    let mut cmd = if has_sudo {
        let mut c = Command::new("sudo");
        c.arg("target/debug/monad-firewall-core");
        c
    } else {
        Command::new("target/debug/monad-firewall-core")
    };

    let status = cmd
        .args(["--interface", interface])
        .status()
        .context("Failed to execute monad-firewall-core runtime engine")?;

    if !status.success() {
        return Err(anyhow!("Firewall daemon exited with an error status"));
    }

    Ok(())
}
