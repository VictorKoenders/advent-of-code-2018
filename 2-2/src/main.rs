use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut previous_lines: Vec<&str> = Vec::new();
    'outer: for line in str.lines() {
        for previous_line in &previous_lines {
            if character_difference(line, previous_line) == 1 {
                println!("The two lines are: ");
                println!(" - {:?}", line);
                println!(" - {:?}", previous_line);
                break 'outer;
            }
        }
        previous_lines.push(line);
    }
}

fn character_difference(s1: &str, s2: &str) -> usize {
    let mut diff_count = 0;
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    for i in 0..s1.len() {
        if s1[i] != s2[i] {
            diff_count += 1;
        }
    }
    diff_count
}
