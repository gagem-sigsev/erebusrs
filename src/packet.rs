#[derive(Debug)]
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

    pub fn calculate_checksum(payload: &str) -> u16 {
        let mut sum = 0;

        let bytes = payload.as_bytes();
        for (_i, &item) in bytes.iter().enumerate() {
            sum += item as u16;
        }
        sum
    }
}
