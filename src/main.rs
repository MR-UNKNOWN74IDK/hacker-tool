mod network;
mod utils;
mod wifi;  // Add this

use std::env;
use std::sync::Arc;
use utils::HackerLogger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let log_path = "/sdcard/hacker_tool.log";
    let mut logger = HackerLogger::new(log_path)?;
    let logger_arc = Arc::new(logger);

    match args.get(1) {
        Some(cmd) if cmd == "scan" => {
            let targets: Vec<String> = (1..255).map(|i| format!("192.168.1.{}", i)).collect();
            network::scan_ips(targets, 80, logger_arc).await;
        }
        Some(cmd) if cmd == "wifi" => {
            if args.len() < 5 {
                println!("Usage: {} wifi <interface> <bssid> <client_mac> <packets>", args[0]);
                return Ok(());
            }
            let iface = &args[2];
            let bssid = &args[3];
            let client = &args[4];
            let packets: usize = args[5].parse().unwrap_or(100);
            wifi::deauth_attack(iface, bssid, client, packets).await;
        }
        _ => {
            println!("Commands:");
            println!("  {} scan          # TCP port scan local subnet", args[0]);
            println!("  {} wifi wlan0 aa:bb:cc:dd:ee:ff 11:22:33:44:55:66 500", args[0]);
        }
    }
    Ok(())
            }
