use std::net::IpAddr;
use std::time::{Duration, Instant};
use socket2::{Domain, Protocol, Socket, Type};

// Public enum for ping output
#[derive(Debug)]
pub enum PingStatus{
    Reply(Duration),
    TimedOut,
    NetworkError(std::io::Error),
}

// function to calculate checksum 
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
    !sum as u16;

}


fn main(){
    println!("main map.rs")
}