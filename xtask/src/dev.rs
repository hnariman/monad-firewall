use std::process::Command;

pub fn dev_env()->anyhow::Result<()>{
    Command::new("podman").args([
        "build",
        "-t",
        "ebpf-firewall-runner",
        ".",
    ]).status()?;

    Command::new("podman").args([
        "run",
        "-it",
        "--rm",
        "--pull=never",
        "--cap-add=CAP_BPF",
        "--cap-add=CAP_NET_ADMIN",
        "--device",
        "/dev/net/tun",
        "localhost/ebpf-firewall-runner:latest",
        "bash"
    ]).status()?;
    Ok(())
}
