use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = env::args().nth(1).unwrap();
    let file = File::open(&file).unwrap();

    let mut lines = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let sum = character_sum(&line);
        lines.push((sum, line));
    }
    'outer: for (index, (sum, line)) in lines.iter().enumerate().skip(1) {
        if index % 1000 == 0 {
            println!("line {}", index);
        }
        for (previous_sum, previous_line) in lines.iter().take(index) {
            if (*sum as isize - *previous_sum as isize).abs() > 1 {
                continue;
            }
            if character_difference(line, previous_line) <= 1 {
                println!("The two lines are: ");
                println!(" - {:?}", line);
                println!(" - {:?}", previous_line);
                break 'outer;
            }
        }
    }
}

fn character_sum(s1: &str) -> usize {
    s1.bytes().map(|b| b as usize).sum()
}

fn character_difference(s1: &str, s2: &str) -> usize {
    let mut diff_count = 0;
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            diff_count += 1;
            if diff_count > 1 {
                return diff_count;
            }
        }
    }
    diff_count
}
