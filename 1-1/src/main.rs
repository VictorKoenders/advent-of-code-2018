use std::env;
use std::io::Read;
use std::fs::File;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut val = 0;

    for line in str.lines() {
        if line.is_empty() {
            continue;
        }

        let direction = &line[..1];
        let amount = &line[1..].parse().unwrap();
        match direction {
            "+" => val += amount,
            "-" => val -= amount,
            x => panic!("Unexpected char: {:?}", x)
        }
    }

    println!("{:?}", val);
}
