mod map;

use std::net::IpAddr;
use std::time::Duration;
use map::{ping, PingStatus}; // Import function and enum

#[tokio::main] 
async fn main() {
    // Define the target (Google's public DNS)
    // .parse() safely converts a string into the IpAddr enum type.
    let target_ip: IpAddr = "8.8.8.8".parse().expect("Invalid IP address format");
    
    // Set timeout limit
    let timeout = Duration::from_secs(2);

    println!("Pinging {} with 9 bytes of data (HelloRust)...", target_ip);

    // Call async function using .await
    let status = ping(target_ip, timeout).await;

    // Use pattern matching to handle enum
    match status {
        PingStatus::Reply(duration) => {
            // duration extracted from enum variant
            println!("Reply from {}: time={:?}", target_ip, duration);
        }
        PingStatus::TimedOut => {
            println!("Request timed out.");
        }
        PingStatus::NetworkError(e) => {
            eprintln!("Network error encountered: {}", e);
            eprintln!("Note: Raw sockets on Windows require Administrator privileges.");
        }
    }
}