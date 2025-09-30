// eBPF imports - uncomment when building eBPF program
// #[cfg(target_os = "linux")]
// use aya::{
//     include_bytes_aligned,
//     programs::{Xdp, XdpFlags},
//     Bpf,
// };
// #[cfg(target_os = "linux")]
// use aya_log::BpfLogger;
use clap::{Parser, Subcommand};
use log::info;
use std::net::Ipv4Addr;

#[cfg(target_os = "linux")]
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

#[derive(Parser)]
#[command(name = "aya-minifirewall")]
#[command(about = "A simple firewall demo using Aya eBPF framework")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Load the firewall program
    Load {
        /// Network interface to attach to
        #[arg(short, long)]
        interface: String,
    },
    /// Block an IP address
    BlockIp {
        /// IP address to block
        ip: String,
    },
    /// Unblock an IP address
    UnblockIp {
        /// IP address to unblock
        ip: String,
    },
    /// Block a port
    BlockPort {
        /// Port number to block
        port: u16,
    },
    /// Unblock a port
    UnblockPort {
        /// Port number to unblock
        port: u16,
    },
    /// List blocked IPs and ports
    List,
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Load { interface } => {
            load_firewall(&interface)?;
        }
        Commands::BlockIp { ip } => {
            block_ip(&ip)?;
        }
        Commands::UnblockIp { ip } => {
            unblock_ip(&ip)?;
        }
        Commands::BlockPort { port } => {
            block_port(port)?;
        }
        Commands::UnblockPort { port } => {
            unblock_port(port)?;
        }
        Commands::List => {
            list_rules()?;
        }
    }

    Ok(())
}

fn load_firewall(interface: &str) -> Result<(), anyhow::Error> {
    #[cfg(target_os = "linux")]
    {
        info!("=== Aya Mini Firewall Demo ===");
        info!("Interface: {}", interface);
        info!("");
        info!("To enable full eBPF functionality:");
        info!("1. Build eBPF program:");
        info!("   cd aya-minifirewall-ebpf");
        info!("   cargo build --release --target bpfel-unknown-none");
        info!("");
        info!("2. Run with root privileges:");
        info!("   sudo ./aya-minifirewall load --interface {}", interface);
        info!("");
        info!("3. Test firewall rules:");
        info!("   sudo ./aya-minifirewall block-ip 192.168.1.100");
        info!("   sudo ./aya-minifirewall block-port 80");
        info!("   sudo ./aya-minifirewall list");
        info!("");
        info!("Current mode: Demo (no actual packet filtering)");
        info!("Press Ctrl+C to exit...");

        // 保持程序运行
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            info!("Shutting down...");
            r.store(false, Ordering::SeqCst);
        })?;

        while running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(100));
        }

        info!("Firewall demo unloaded");
    }

    #[cfg(not(target_os = "linux"))]
    {
        info!("eBPF firewall is only supported on Linux. This is a demo mode.");
        info!("Interface: {}", interface);
        info!("Firewall would be loaded here on Linux.");
    }

    Ok(())
}

fn block_ip(ip: &str) -> Result<(), anyhow::Error> {
    let ip_addr: Ipv4Addr = ip.parse()?;
    let ip_u32 = u32::from(ip_addr);

    // In a real implementation, you would interact with the eBPF maps here
    // For this demo, we'll just print the action
    info!("Blocking IP: {} ({})", ip, ip_u32);
    Ok(())
}

fn unblock_ip(ip: &str) -> Result<(), anyhow::Error> {
    let ip_addr: Ipv4Addr = ip.parse()?;
    let ip_u32 = u32::from(ip_addr);

    info!("Unblocking IP: {} ({})", ip, ip_u32);
    Ok(())
}

fn block_port(port: u16) -> Result<(), anyhow::Error> {
    info!("Blocking port: {}", port);
    Ok(())
}

fn unblock_port(port: u16) -> Result<(), anyhow::Error> {
    info!("Unblocking port: {}", port);
    Ok(())
}

fn list_rules() -> Result<(), anyhow::Error> {
    info!("Blocked IPs: (demo - no actual rules loaded)");
    info!("Blocked Ports: (demo - no actual rules loaded)");
    Ok(())
}
