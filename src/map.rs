use std::io::Read;
use std::net::IpAddr;
use std::ops::ControlFlow::Continue;
use std::time::{Duration, Instant};
use etherparse::ip_number::ICMP;
use pnet::packet::icmp;
use socket2::{Domain, Protocol, Socket, Type};
use std::collections::HashSet;

// Public enum for ping output
#[derive(Debug)]
pub enum PingStatus{
    Reply(Duration),
    TimedOut,
    NetworkError(std::io::Error),
}

// Public enum for Broadcast output
#[derive(Debug)]
pub enum BroadcastStatus {
    Completed(HashSet<IpAddr>),
    NetworkError(std::io::Error),
}


// ---- CHECKSUM FUNCTION ----
fn calculate_checksum(data: &[u8]) -> u16 {
    // takes slice of bytes and returns 16-bit integer

    // takes array of bytes and splits them into pairs
    let mut sum = 0u32;
    let mut chunks = data.chunks_exact(2);

    // ensures bytes are read in correct network order (big endian)
    // combines two 8-bit bytes into a 16-bit "word"
    for chunk in chunks.by_ref() {
        let word = u16::from_be_bytes([chunk[0], chunk[1]]);
        sum += word as u32;
    }

    // if packet has odd number of bytes, leftover bytes are left in a remainder. If so, it pads with zeros and adds it to a sum
    if let Some(&byte) = data.chunks_exact(2).remainder().first() {
        sum += (byte as u32) << 8;
    }

    // One's complement
    // if sum overflowed by 16 bits, takes extra bits from left and adds them to the right side (sum & 0xffff)
    while (sum >> 16) > 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }

    // flip all the bits (!) to finalize checksum
    !sum as u16

} // ---- END CHECKSUM FUNCTION ----

// ---- ICMP PACKET PAYLOAD FUNCTION ----
fn icmp_packet() -> Vec<u8> {
    let mut packet: Vec<u8> = vec![8, 0, 0, 0, 0x12, 0x34, 0x00, 0x01];

    packet.extend_from_slice(b"HelloRust");

    // calculate checksum of packet
    let checksum = calculate_checksum(&packet);

    // slice notation - grab 3rd and 4th bytes adn overwrite with checksum value
    packet[2..4].copy_from_slice(&checksum.to_be_bytes());

    packet
}

// ---- PING FUNCTION ----
pub async fn ping(target: IpAddr, timeout: Duration) -> PingStatus {
    let domain = match target {
        IpAddr::V4(_) => Domain::IPV4,
        IpAddr::V6(_) => Domain::IPV6,
    };

    // Type::RAW -> Not TCP or UDP | Raw IP payload
    let mut socket = match Socket::new(domain, Type::RAW, Some(Protocol::ICMPV4)) {
        Ok(s) => s,
        Err(e) => return PingStatus::NetworkError(e),
    };

    let packet = icmp_packet();

    // sending packet
    // convert Rust standard IPAddr into socket2 format
    let addr = socket2::SockAddr::from(std::net::SocketAddr::new(target, 0));
    // start timer
    let start_time = Instant::now();

    if let Err(e) = socket.send_to(&packet, &addr) {
        return PingStatus::NetworkError(e);
    }

    // Receive loop
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        if start_time.elapsed() > timeout {
            return PingStatus::TimedOut;
        }

        match socket.read(&mut buf) {
            Ok(bytes_read) => {
                let ip_header_len = 20;
                if bytes_read > ip_header_len {
                    let icmp_packet = &buf[ip_header_len..bytes_read];
                    if icmp_packet.len() > 1 && icmp_packet[0] == 0 {
                        return PingStatus::Reply(start_time.elapsed());
                    }
                }
            }

            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => return PingStatus::NetworkError(e),
        }
    }

} // ---- END PING FUNCTION ----

// ---- BROADCAST FUNCTION ----
pub fn broadcast(target: IpAddr, timeout: Duration) -> BroadcastStatus {
    // braodcast address
    let global_address = "255.255.255.255";

    // create raw socket for broadcast
    let mut domain: Domain = match target {
        IpAddr::V4(_) => Domain::IPV4,
        IpAddr::V6(_) => Domain::IPV6,
    };

    let mut socket = match Socket::new(domain, Type::RAW, Some(Protocol::ICMPV4)) {
        Ok(s) => s,
        Err(e) => return BroadcastStatus::NetworkError(e),
    };

    let mut packet: Vec<u8> = icmp_packet();

    socket.set_broadcast(true);

    let addr = socket2::SockAddr::from(std::net::SocketAddr::new(target, 0));

    let start_time: Instant = Instant::now();

    // send packet
    if let Err(e) = socket.send_to(&packet, &addr) {
        return BroadcastStatus::NetworkError(e);
    }



    // change 
    return BroadcastStatus::NetworkError("Error");


} // ---- END BROADCAST FUNCTION ----
