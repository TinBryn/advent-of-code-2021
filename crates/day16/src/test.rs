use bitvec::prelude::*;

use crate::{packet::{Packet, from_bits}, problem::Problem};

const EXAMPLE: &str = "8A004A801A8002F478";

const EXAMPLE_1: &str = "8A004A801A8002F478";

#[test]
fn parse_input() {
    let input = EXAMPLE.parse().unwrap();
    let problem = Problem::from_input(input);
    assert_eq!(problem.to_string().as_str(), EXAMPLE);
}

#[test]
fn literal_packet() {
    let input = "D2FE28".parse().unwrap();
    let problem = Problem::from_input(input);
    let (packet, size) = Packet::get_packet(problem.data.as_bitslice());

    println!("{:#?}\nsize: {}", packet, size);
}

#[test]
fn operator_packet_len_type0() {
    let input = "38006F45291200".parse().unwrap();
    let problem = Problem::from_input(input);
    let bits = &problem.data[..];
    let (packet, size) = Packet::get_packet(bits);

    println!("{:#?}\nsize: {}", packet, size);
}

#[test]
fn operator_packet_len_type1() {
    let input = "EE00D40C823060".parse().unwrap();
    let problem = Problem::from_input(input);
    let bits = &problem.data[..];
    let (packet, size) = Packet::get_packet(bits);

    println!("{:#?}\nsize: {}", packet, size);
}

#[test]
fn convert_from_bits() {
    let input: BitVec = bitvec![1, 0, 1, 0, 1];
    let value = from_bits(input.as_bitslice());
    assert_eq!(value, 1 + 4 + 16)
}

#[test]
fn problem1_exp1() {
    let input = EXAMPLE_1.parse().unwrap();
    let problem = Problem::from_input(input);
    println!("{}", problem);
    let actual = problem.part1();
    let expected = 16;
    assert_eq!(expected, actual);
}

#[test]
fn problem2() {
    let input = EXAMPLE.parse().unwrap();
    let problem = Problem::from_input(input);
    let actual = problem.part2();
    let expected = 15;
    assert_eq!(expected, actual);
}
