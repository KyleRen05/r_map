# R_MAP

R_MAP is a network mapper and a packet sniffer tool built in Rust!

# Dependencies

- color-eyre = "0.6.5"
- crossterm = "0.29.0"
- etherparse = "0.21.0"
- pcap = "2.5.0"
- pnet = "0.35.0"
- ratatui = "0.30.2"
- socket2 = "0.6.5"
- surge-ping = "0.9.1"
- tokio = "1.53.2"

# Understanding Network Mapping

The point of network mapping is to create a map of all the devices connected to a network, how components interact with each other, and what could be causing failure in a network.

- Newtork Discovery
- Relationship Identification
- Network Model
- Logical Map
- Up-to-date Information

# map.rs

## Helper Functions

> ### caclulate_checksum
>
> Checksum algorithms are used to detect errors, tampering, or irregularities in data sent over a network.<br/>The purpose of this helper function is to calculate the checksum for the ICMP packet being sent to see if the packets were correctly received by the target IP address.

> ### pub async fn ping
>
> ICMP Header<br/>
<ol>
<li>Type (1 byte): 8 means "Echo Request".</li>
<li>Code (1 byte): 0 is standard for pings.</li>
<li>Checksum (2 bytes): 0, 0 (We set this to zero initially before we calculate the real checksum).</li>
<li>Identifier (2 bytes): 0x12, 0x34 (An arbitrary ID to help us identify our specific ping).</li>
<li>Sequence (2 bytes): 0x00, 0x01 (Keeps track of which ping this is, e.g., ping #1).</li>
</ol>

