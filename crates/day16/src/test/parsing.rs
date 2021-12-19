use crate::{
    packet::{Data, Operator, Packet, from_bits},
    problem::Problem,
};

use bitvec::prelude::*;

#[test]
fn parse_input_to_string() {
    let example = "8A004A801A8002F478";
    let input = example.parse().unwrap();
    let problem = Problem::from_input(input);
    assert_eq!(problem.to_string().as_str(), example);
}

#[test]
fn literal_packet() {
    let input = "D2FE28".parse().unwrap();
    let problem = Problem::from_input(input);
    let (packet, _) = Packet::get_packet(problem.data.as_bitslice()).unwrap();

    let expected = Packet {
        version: 6,
        data: Data::Literal(2021),
    };

    assert_eq!(packet, expected);
}

#[test]
fn operator_packet_len_type0() {
    let input = "38006F45291200".parse().unwrap();
    let problem = Problem::from_input(input);
    let bits = &problem.data[..];
    let (packet, _) = Packet::get_packet(bits).unwrap();

    let expected = Packet {
        version: 1,
        data: Data::Operator(
            Operator::Less,
            vec![
                Packet {
                    version: 6,
                    data: Data::Literal(10),
                },
                Packet {
                    version: 2,
                    data: Data::Literal(20),
                },
            ],
        ),
    };

    assert_eq!(packet, expected);
}

#[test]
fn operator_packet_len_type1() {
    let input = "EE00D40C823060".parse().unwrap();
    let problem = Problem::from_input(input);
    let bits = &problem.data[..];
    let (packet, _) = Packet::get_packet(bits).unwrap();

    let expected = Packet {
        version: 7,
        data: Data::Operator(
            Operator::Max,
            vec![
                Packet {
                    version: 2,
                    data: Data::Literal(1),
                },
                Packet {
                    version: 4,
                    data: Data::Literal(2),
                },
                Packet {
                    version: 1,
                    data: Data::Literal(3),
                },
            ],
        ),
    };

    assert_eq!(packet, expected);
}

#[test]
fn convert_from_bits() {
    let input: BitVec = bitvec![1, 0, 1, 0, 1];
    let value = from_bits(input.as_bitslice());
    assert_eq!(value, 21)
}
