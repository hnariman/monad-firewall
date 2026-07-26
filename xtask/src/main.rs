use anyhow::{Result};
use clap::{Parser, Subcommand};

mod daemon;
mod userspace;
mod bpf;
mod dev;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Automation orchestrator for monad-firewall")]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Compile the eBPF bytecode and the user-space agent
    Build,
    /// Run the core firewall daemon with escalated privileges
    Run {
        #[arg(short, long, default_value = "eth0")]
        interface: String,
    },
    Dev
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            bpf::build_ebpf()?;
            userspace::build_userspace()?;
        }
        Commands::Run { interface } => {
            bpf::build_ebpf()?;
            userspace::build_userspace()?;
            daemon::run_daemon(&interface)?;
        }
        Commands::Dev => {
            dev::dev_env()?;
        },

    }

    Ok(())
}

