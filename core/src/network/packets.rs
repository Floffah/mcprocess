use std::io::{BufReader, Cursor};

pub trait Packet {
    fn serialize(&self) -> Vec<u8>;
    fn get_id(&self) -> PacketId;
}

pub struct RawPacket {
    pub id: u8,
    pub data: Vec<u8>,
}

impl RawPacket {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.full_len().to_be_bytes().iter());
        buf.push(self.id);
        buf.extend(self.data.iter());
        buf
    }

    pub fn deserialize(len: u32, buf: &[u8]) -> Self {
        let id = buf[4];
        let data = buf[5..].to_vec();
        RawPacket {
            id,
            data,
        }
    }

    pub fn get_packet(&self) -> Box<dyn Packet> {
        match self.id {
            0x00 => Box::new(IdentifyPacket::deserialize(self)),
            _ => panic!("Unknown packet id"),
        }
    }

    pub fn data_len(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn full_len(&self) -> u32 {
        self.data_len() + 1
    }
}

pub enum PacketId {
    Identify = 0x00,
}

pub struct IdentifyPacket {
    pub client_type: String,
}

impl IdentifyPacket {
    pub fn deserialize(buf: &RawPacket) -> Self {
        let client_type = String::from_utf8_lossy(&buf.data).to_string();
        IdentifyPacket {
            client_type,
        }
    }
}

impl Packet for IdentifyPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.client_type.as_bytes());
        buf
    }

    fn get_id(&self) -> PacketId {
        PacketId::Identify
    }
}

