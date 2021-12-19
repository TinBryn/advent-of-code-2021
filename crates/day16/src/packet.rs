use std::fmt::Debug;

use bitvec::prelude::*;

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub data: PacketData,
}

pub fn get_literal(mut bits: &BitSlice) -> (usize, usize) {
    let mut lit = 0;
    let mut size = 0;
    loop {
        lit <<= 4;
        lit += from_bits(&bits[1..5]);
        // println!("{}", lit);
        if !bits[0] {
            break;
        }
        bits = &bits[5..];
        size += 5;
    }

    (lit, size + 5)
}

pub fn get_sub_packets(bits: &BitSlice) -> (Vec<Packet>, usize) {
    if !bits[0] {
        let bits = &bits[1..];
        let len = from_bits(&bits[0..15]);
        let bits = &bits[15..];
        let packets = get_packets_to_length(bits, len);
        (packets, len + 16)
    } else {
        let bits = &bits[1..];
        let count = from_bits(&bits[0..11]);
        let bits = &bits[11..];
        let (packets, size) = get_n_packets(bits, count);
        (packets, size + 12)
    }
}

fn get_n_packets(mut bits: &BitSlice, count: usize) -> (Vec<Packet>, usize) {
    let mut result = Vec::with_capacity(count);
    let mut size = 0;
    for _ in 0..count {
        let (packet, s) = Packet::get_packet(bits);
        size += s;
        bits = &bits[s..];
        result.push(packet);
    }
    (result, size)
}

fn get_packets_to_length(mut bits: &BitSlice, len: usize) -> Vec<Packet> {
    let mut result = vec![];
    let mut count = 0;
    while count < len {
        let (packet, size) = Packet::get_packet(bits);
        count += size;
        bits = &bits[size..];
        result.push(packet);
    }
    result
}

impl Packet {
    pub fn get_packet(bits: &BitSlice) -> (Self, usize) {
        let header = Header::from(&bits[0..6]);
        let bits = &bits[6..];
        let (data, size) = if header.type_id == 4 {
            let (lit, size) = get_literal(bits);
            (PacketData::Literal(lit), size + 6)
        } else {
            let (packets, size) = get_sub_packets(bits);
            (PacketData::Operator(packets), size + 6)
        };

        (Self { header, data }, size)
    }

    pub fn eval(&self) -> usize {
        match &self.data {
            PacketData::Literal(lit) => *lit,
            PacketData::Operator(packets) => {
                let results = packets.iter().map(Packet::eval);
                match self.header.type_id {
                    0 => results.sum(),
                    1 => results.product(),
                    2 => results.min().unwrap_or_default(),
                    3 => results.max().unwrap_or_default(),
                    5 | 6 | 7 => {
                        let results: Vec<_> = results.collect();
                        if results.len() >= 2 {
                            match self.header.type_id {
                                5 => (results[0] > results[1]) as usize,
                                6 => (results[0] < results[1]) as usize,
                                7 => (results[0] == results[1]) as usize,
                                _ => unreachable!(),
                            }
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Header {
    #[allow(dead_code)]
    pub version: usize,
    pub type_id: usize,
}

impl From<&BitSlice> for Header {
    fn from(bits: &BitSlice) -> Self {
        let version = from_bits(&bits[0..3]);

        let type_id = from_bits(&bits[3..6]);

        Self { version, type_id }
    }
}

#[derive(Debug)]
pub enum PacketData {
    Literal(usize),
    Operator(Vec<Packet>),
}

pub fn from_bits(bits: &BitSlice) -> usize {
    let mut result = 0;
    for bit in bits {
        result <<= 1;
        result += *bit as usize;
    }
    result
}
