use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use crate::utils::HackerLogger;

pub async fn scan_ips(ips: Vec<String>, port: u16, logger: Arc<HackerLogger>) {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(100)); // Mobile-friendly concurrency

    let mut handles = vec![];

    for ip in ips {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let logger_clone = logger.clone();
        let addr = format!("{}:{}", ip, port);

        handles.push(tokio::spawn(async move {
            let _permit = permit;
            match timeout(Duration::from_millis(300), TcpStream::connect(&addr)).await {
                Ok(Ok(_)) => {
                    if let Err(e) = logger_clone.log(&format!("OPEN: {} {}", addr, chrono::Local::now().format("%H:%M:%S"))) {
                        eprintln!("Log error: {}", e);
                    }
                }
                _ => {}
            }
        }));
    }

    futures::future::join_all(handles).await;
    }
