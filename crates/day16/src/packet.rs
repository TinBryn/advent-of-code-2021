use std::fmt::Debug;

use bitvec::prelude::*;

#[derive(Debug)]
pub enum Error {
    TypeId(usize)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet {
    pub version: usize,
    pub data: Data,
}

pub fn get_literal(mut bits: &BitSlice) -> (usize, usize) {
    let mut lit = 0;
    let mut size = 0;
    loop {
        lit <<= 4;
        lit += from_bits(&bits[1..5]);
        if !bits[0] {
            break;
        }
        bits = &bits[5..];
        size += 5;
    }

    (lit, size + 5)
}

pub fn get_sub_packets(bits: &BitSlice) -> Result<(Vec<Packet>, usize), Error> {
    if !bits[0] {
        let bits = &bits[1..];
        let len = from_bits(&bits[0..15]);
        let bits = &bits[15..];
        let packets = get_packets_to_length(bits, len)?;
        Ok((packets, len + 16))
    } else {
        let bits = &bits[1..];
        let count = from_bits(&bits[0..11]);
        let bits = &bits[11..];
        let (packets, size) = get_n_packets(bits, count)?;
        Ok((packets, size + 12))
    }
}

fn get_n_packets(mut bits: &BitSlice, count: usize) -> Result<(Vec<Packet>, usize), Error> {
    let mut result = Vec::with_capacity(count);
    let mut size = 0;
    for _ in 0..count {
        let (packet, s) = Packet::get_packet(bits)?;
        size += s;
        bits = &bits[s..];
        result.push(packet);
    }
    Ok((result, size))
}

fn get_packets_to_length(mut bits: &BitSlice, len: usize) -> Result<Vec<Packet>, Error> {
    let mut result = vec![];
    let mut count = 0;
    while count < len {
        let (packet, size) = Packet::get_packet(bits)?;
        count += size;
        bits = &bits[size..];
        result.push(packet);
    }
    Ok(result)
}

impl Packet {
    pub fn get_packet(bits: &BitSlice) -> Result<(Self, usize), Error> {
        let header = Header::from(&bits[0..6]);
        let version = header.version;
        let type_id = header.type_id;
        let bits = &bits[6..];
        let (data, size) = if header.type_id == 4 {
            let (lit, size) = get_literal(bits);
            (Data::Literal(lit), size + 6)
        } else {
            let (packets, size) = get_sub_packets(bits)?;
            (Data::Operator(type_id.try_into()?, packets), size + 6)
        };

        Ok((Self { version, data }, size))
    }

    pub fn eval(&self) -> usize {
        self.data.eval()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl TryFrom<usize> for Operator {
    type Error = Error;
    fn try_from(type_id: usize) -> Result<Self, Self::Error> {
        let v = match type_id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            _ => return Err(Error::TypeId(type_id)),
        };
        Ok(v)
    }
}

impl Operator {
    pub fn eval(&self, evals: impl Iterator<Item = usize>) -> usize {
        match *self {
            Operator::Sum => evals.sum(),
            Operator::Product => evals.product(),
            Operator::Min => evals.min().unwrap_or_default(),
            Operator::Max => evals.max().unwrap_or_default(),
            Operator::Greater => Self::check(evals, |a, b| a > b),
            Operator::Less => Self::check(evals, |a, b| a < b),
            Operator::Equal => Self::check(evals, |a, b| a == b),
        }
    }

    fn check<F: Fn(usize, usize) -> bool>(mut evals: impl Iterator<Item = usize>, f: F) -> usize {
        evals
            .next()
            .and_then(|a| evals.next().map(|b| f(a, b) as usize))
            .unwrap_or_default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Data {
    Literal(usize),
    Operator(Operator, Vec<Packet>),
}

impl Data {
    pub fn eval(&self) -> usize {
        match self {
            Data::Literal(lit) => *lit,
            Data::Operator(op, packets) => op.eval(packets.iter().map(Packet::eval)),
        }
    }
}

pub fn from_bits(bits: &BitSlice) -> usize {
    let mut result = 0;
    for bit in bits {
        result <<= 1;
        result += *bit as usize;
    }
    result
}
