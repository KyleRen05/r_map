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
<hr/>

> ## Helper Functions
>
>> ### caclulate_checksum
>>
>> Checksum algorithms are used to detect errors, tampering, or irregularities in data sent over a network.<br/>The purpose of this helper function is to calculate the checksum for the ICMP packet being sent to see if the packets were correctly received by the target IP address.

