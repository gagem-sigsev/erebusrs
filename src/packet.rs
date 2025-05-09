use pnet::util::checksum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub src: String,
    pub dst: String,
    pub data_length: u32,
    pub checksum: u16,
    pub payload: String,
}

impl Packet {
    pub fn new(
        src: String,
        dst: String,
        payload: String,
        data_length: u32,
        checksum: u16,
    ) -> Packet {
        Packet {
            src,
            dst,
            payload,
            checksum,
            data_length,
        }
    }

    // TODO: implement custom checksum calculation
    pub fn calculate_checksum(payload: &str) -> u16 {
        let checksum = checksum(payload.as_bytes(), 0);
        checksum
    }

    // TODO: implement custom serialization
    pub fn serialize_packet(&self) -> Vec<u8> {
        bincode::serialize(&self).expect("Unable to serialize packet")
    }

    // TODO: implement custom deserialization
    pub fn deserialize_packet(&self, spkt: &[u8]) -> Packet {
        bincode::deserialize(spkt).expect("Unable to deserialize packet")
    }
}
