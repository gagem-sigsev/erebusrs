use crate::packet::Packet;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
#[derive(Debug)]
pub struct Erebus {
    packets_sent: u32,
    packets_recieved: u32,
    packets_total: u32,
}

impl Erebus {
    pub fn new() -> Erebus {
        Erebus {
            packets_sent: 0,
            packets_recieved: 0,
            packets_total: 0,
        }
    }
    pub fn log_packet(&self, pkt: &Packet) {
        println!(
            "Src: {}\nDst: {}\nPayload: {}\nChecksum: {}\nLength: {}",
            pkt.src, pkt.dst, pkt.payload, pkt.checksum, pkt.data_length
        );
    }

    pub fn log_protocol(&self) {
        println!(
            "Packets sent: {}\nPackets recieved: {}\nTotal packet: {}\n",
            self.packets_sent, self.packets_recieved, self.packets_total,
        );
    }

    pub fn send_packet(&self, spkt: &Packet) -> std::io::Result<()> {
        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
        let listener = TcpListener::bind(addr)?;
        let (mut stream, clientaddr) = listener.accept().expect("Unable to accept client");
        println!("Client: {:?} has connected...", clientaddr);
        let spkt = spkt.serialize_packet();
        stream.write_all(&spkt[0..spkt.len()])?;
        Ok(())
    }
}
