# Advent of Code 2021

[![Day1](https://github.com/TinBryn/advent-of-code-2021/actions/workflows/day1.yml/badge.svg)](https://github.com/TinBryn/advent-of-code-2021/actions/workflows/day1.yml)

[![Day2](https://github.com/TinBryn/advent-of-code-2021/actions/workflows/day2.yml/badge.svg)](https://github.com/TinBryn/advent-of-code-2021/actions/workflows/day2.yml)

[![Day3](https://github.com/TinBryn/advent-of-code-2021/actions/workflows/day3.yml/badge.svg)](https://github.com/TinBryn/advent-of-code-2021/actions/workflows/day3.yml)

## 🎄 Merry Christmas fellow Rustacean 🎄

It's that time of year again, Santa 🎅 is in trouble and the elves needs our help to save Christmas. For such an
important task we need the utmost in reliability and performance so I will be using Rust to do my
part. So get ready to `.unwrap()` those 🎁 to save Christmas and don't `panic!()`.

## Structure

This is setup as a Cargo workspace with one crate for each day's problem, plus any additional
common requirements that arise (such as 2019's intcode computer). Each crate will be a binary and
can all be executed simply via `cargo run` or a specific day via `cargo run -p day*`.

There are some tests that I will be writing based on examples given in the descriptions, these can
also be run as above using `cargo test` instead of `cargo run`.
