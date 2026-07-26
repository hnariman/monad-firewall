use anyhow::{Context, Result, anyhow};
use std::process::Command;
use which::which;

pub fn build_ebpf() -> Result<()> {
    println!("⚙️ Compiling monad-firewall-ebpf targeting bpfeb-unknown-none...");

    let bpf_linker_found = which("bpf-linker").is_ok()
        || std::env::var("HOME")
            .map(|h| {
                std::path::Path::new(&h)
                    .join(".cargo/bin/bpf-linker")
                    .exists()
            })
            .unwrap_or(false);

    if !bpf_linker_found {
        return Err(anyhow!(
            "Missing 'bpf-linker'. Please install it using: cargo install bpf-linker"
        ));
    }

    let status = Command::new("cargo")
        .args([
            "+nightly",
            "build",
            "--package",
            "monad-firewall-ebpf",
            "--target",
            "bpfel-unknown-none",
            "--release",
            "-Z",
            "build-std=core",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::inherit())
        .status()
        .context("Failed to execute cargo build for eBPF bytecode")?;

    if !status.success() {
        return Err(anyhow!("eBPF bytecode compilation failed"));
    }

    Ok(())
}

