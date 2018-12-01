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

fn part1(values: &Vec<i32>) {
    let mut freq :i32 = 0;
    for val in values {
        freq = freq + val;
    }
    println!("Part1 result = {}", freq);
}


fn read_file_to_vec(file: BufReader<&File>) -> Vec<i32> {
    let mut res = Vec::new();
    for line in file.lines() {
        let l = line.unwrap();
        res.push(parse_and_apply(&l));
    }
    res
}


fn part2(values : &Vec<i32>) {
    let mut freq_map = HashMap::new();
    let mut freq = 0;
    freq_map.insert(freq, 0);
    while true {
        for val in values {
            freq = freq + val;
            if freq_map.contains_key(&freq) {
                println!("Part2 result = {}", freq);
                return;
            }
            freq_map.insert(freq, 0);
        }
    }
}

fn main() {
    let f = File::open("input.txt").expect("file not found");
    let file = BufReader::new(&f);
    let values = read_file_to_vec(file);
    part1(&values);
    part2(&values);
}
