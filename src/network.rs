use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::sync::Arc;

pub async fn scan_network(targets: Vec<String>, port: u16) {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(500)); // Limits to 500 connections at once

    for ip in targets {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        
        tokio::spawn(async move {
            let _permit = permit;
            let addr = format!("{}:{}", ip, port);
            
            if let Ok(_) = timeout(Duration::from_millis(500), TcpStream::connect(&addr)).await {
                println!("[+] Found open port: {}", addr);
            }
        });
    }
                             }
