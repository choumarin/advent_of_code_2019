use std::io::{BufRead, BufReader, Read};
use std::fs::File;

mod day01;
mod day02;

fn main() {
    let file = File::open("src/day01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| {x.unwrap().parse().unwrap()});
    println!("day01_a answer: {}", day01::entry_a(lines));

    let file = File::open("src/day01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| {x.unwrap().parse().unwrap()});
    println!("day01_b answer: {}", day01::entry_b(lines));

    let mut file = File::open("src/day02/input.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();
    let mut line = line.trim().to_string();
    println!("day02_a answer: {}", day02::entry_a(line));

    let mut file = File::open("src/day02/input.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();
    let mut line = line.trim().to_string();
    println!("day02_b answer: {}", day02::entry_b(line));
}