use std::fs::File;
use std::io::{BufRead, BufReader, Read};

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let file = File::open("src/day01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap().parse().unwrap());
    println!("day01_a answer: {}", day01::entry_a(lines));

    let file = File::open("src/day01/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap().parse().unwrap());
    println!("day01_b answer: {}", day01::entry_b(lines));

    let mut file = File::open("src/day02/input.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();
    let line = line.trim().to_string();
    println!("day02_a answer: {}", day02::entry_a(line));

    let mut file = File::open("src/day02/input.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();
    let line = line.trim().to_string();
    println!("day02_b answer: {}", day02::entry_b(line));

    let mut file = File::open("src/day03/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let content = content.trim().to_string();
    println!("day03_a answer: {}", day03::entry_a(content));

    let mut file = File::open("src/day03/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let content = content.trim().to_string();
    println!("day03_b answer: {}", day03::entry_b(content));

    let mut file = File::open("src/day04/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let content = content.trim().to_string();
    println!("day04_a answer: {}", day04::entry_a(content));

    let mut file = File::open("src/day04/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let content = content.trim().to_string();
    println!("day04_b answer: {}", day04::entry_b(content));
}
