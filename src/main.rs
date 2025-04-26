mod packet;
mod protocol;

use crate::packet::Packet;
use crate::protocol::Erebus;

fn main() {
    let src = String::from("127.0.0.1");
    let dst = String::from("127.0.0.1");
    let payload = String::from("Erebus packet!\n");
    let checksum = Packet::calculate_checksum(&payload);
    let data_length = payload.len() as u32;
    let packet: Packet = Packet::new(src, dst, payload, data_length, checksum);

    let mut erebus = Erebus::new();
    erebus.log_protocol();
    erebus.log_packet(&packet);

    let spkt = packet.serialize_packet();
    println!("Serialized packet: {:?}\n", spkt);

    let dspkt = packet.deserialize_packet(&spkt);
    println!("Deseralized packet: {:?}\n", dspkt);

    let res = erebus.send_packet(&packet);

    println!("Send result: {:?}", res);
}
