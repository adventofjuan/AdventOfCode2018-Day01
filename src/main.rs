use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_and_apply(l: &String) -> i32{
    if &l[0..1] == "+" {
        let num = &l[1..].parse::<i32>().unwrap();
        return *num;
    } else if &l[0..1] == "-" {
        let num = &l[1..].parse::<i32>().unwrap();
        return -*num;
    }
    0
}

fn part1(f_lines: BufReader<&File>) {
    let mut freq :i32 = 0;
    for line in f_lines.lines() {
        let l = line.unwrap();
        freq = freq + parse_and_apply(&l)
    }
    println!("Freq = {}", freq);
}


fn part2(f_lines: BufReader<&File>) {
    let mut freq_map = HashMap::new();
    let mut freq = 0;
    freq_map.insert(freq, 0);
    for line in f_lines.lines() {
        let l = line.unwrap();
        freq = freq + parse_and_apply(&l)
}

fn main() {
    let f = File::open("input.txt").expect("file not found");
    let file = BufReader::new(&f);
    part1(file)
}
