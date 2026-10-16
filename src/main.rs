mod map;

use std::collections::HashSet;
use std::io;
use std::net::IpAddr;
use std::process::exit;
use std::time::Duration;
use map::*;

async fn ping_main(){
    // .parse() safely converts a string into the IpAddr enum type.
    let mut t_ip: String = String::new();
    println!("Please enter a valid IP address");
    io::stdin().read_line(&mut t_ip).expect("Failed to read IP Addr");

    let target_ip: IpAddr = t_ip.trim().parse().expect("Invalid IP address format");
    
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

async fn broadcast_main(){

    let mut broadcast_ip: String = String::new();
    broadcast_ip = "192.168.0.255".to_string();

    let target_ip: IpAddr = broadcast_ip.trim().parse().expect("Could not parse broadcast ip");

    let timeout: Duration = Duration::from_secs(6);

    println!("Sending broadcast for IP enumeration\n");

    let bc = broadcast(target_ip, timeout).await;

    match bc {
        BroadcastStatus::Completed(discovered_ips) => {
            println!("Broadcast complete. Found {} devices!\n", discovered_ips.len());

            for ip in discovered_ips {
                println!("- {}", ip)
            }
        }

        BroadcastStatus::NetworkError(e) => {
            eprintln!("Broadcast failed {}", e);
        }
    }
}

async fn ping_sweep_main(){
    let octet3 = [192, 168, 0];

    println!("Performing Ping Sweep on local network: {}.{}.{}.x\n", octet3[0], octet3[1], octet3[2]);

    let timeout: Duration = Duration::from_secs(2);

    let active_hosts = ping_sweep(octet3, timeout).await;

    println!("Sweep complete\nFound {} devices!\n", active_hosts.len());

    for (ip, duration) in active_hosts {
        println!("- {} (response time: {}ms)", ip, duration.as_millis());
    }
}

#[tokio::main] 
async fn main() {

    let mut option: String = String::new();

    println!("Please choose an option:\n");
    println!("(1) ping\n(2) Broadcast ICMP\n(3) Ping Sweep\n");

    io::stdin().read_line(&mut option).expect("Could not read option");

    if option.trim() == "1" {
        ping_main().await;
    }
    else if option.trim() == "2" {
        broadcast_main().await;
    }
    else if option.trim() == "3" {
        ping_sweep_main().await;
    }
    else{
        return
    }
}