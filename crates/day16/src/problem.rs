use std::fmt::Display;

use crate::{input::Input, packet::Packet};

#[derive(Debug, Clone)]
pub struct Problem {
    pub data: bitvec::vec::BitVec,
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in
            self.data.as_bitslice().chunks(4).map(|w| {
                ((w[0] as u8) << 3) + ((w[1] as u8) << 2) + ((w[2] as u8) << 1) + w[3] as u8
            })
        {
            write!(f, "{}", u8_to_hex(byte) as char)?;
        }

        Ok(())
    }
}

impl Problem {
    pub fn from_input(input: Input) -> Self {
        let data = input
            .line
            .data
            .as_bytes()
            .iter()
            .map(|&b| hex_to_u8(b))
            .flat_map(|n| [(n & 0x8) >> 3, (n & 0x4) >> 2, (n & 0x2) >> 1, n & 0x1])
            .map(|b| b > 0)
            .collect();
        Self { data }
    }

    pub fn part1(&self) -> usize {
        let packet = self.common();
        sum_version_numbers(&packet)
    }

    pub fn part2(&self) -> usize {
        let packet = self.common();
        packet.eval()
    }

    pub fn common(&self) -> Packet {
        let bits = self.data.as_bitslice();
        Packet::get_packet(bits).0
    }
}

fn sum_version_numbers(packet: &Packet) -> usize {
    let sub = match &packet.data {
        crate::packet::PacketData::Literal(_) => 0usize,
        crate::packet::PacketData::Operator(packets) => {
            packets.iter().map(sum_version_numbers).sum()
        }
    };

    sub + packet.header.version
}

fn hex_to_u8(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'A'..=b'F' => c - b'A' + 10,
        _ => todo!(),
    }
}

fn u8_to_hex(c: u8) -> u8 {
    match c {
        0..=9 => c + b'0',
        10..=15 => c - 10 + b'A',
        _ => todo!(),
    }
}
