mod network;

#[tokio::main]
async fn main() {
    println!("[*] Starting fast scan engine...");

    // Example: Scanning a local subnet
    let mut targets = Vec::new();
    for i in 1..255 {
        targets.push(format!("192.168.1.{}", i));
    }

    // Scan for HTTP (port 80)
    network::scan_network(targets, 80).await;
    
    println!("[*] Scan complete.");
}
