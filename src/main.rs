mod packet;
mod protocol;

use crate::packet::Packet;
use crate::protocol::Erebus;

fn main() {
    let src = String::from("127.0.0.1");
    let dst = String::from("127.0.0.1");
    let payload = String::from("Hello, network!");
    let checksum = Packet::calculate_checksum(&payload);
    let data_length = payload.len() as u32;
    let packet: Packet = Packet::new(src, dst, payload, data_length, checksum);

    let erebus = Erebus::new();
    erebus.log_protocol();
    erebus.log_packet(&packet);
}
