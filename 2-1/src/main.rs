use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut two_count = 0;
    let mut three_count = 0;
    for line in str.lines() {
        let mut bytes = line.as_bytes().to_vec();
        bytes.sort();

        let mut i = 0;
        let mut has_two = false;
        let mut has_three = false;
        while i < bytes.len()  && (!has_two || !has_three) {
            let mut j = i + 1;
            while j < bytes.len() && bytes[i] == bytes[j] {
                j += 1;
            }
            let diff = j - i;
            if diff == 2 {
                has_two = true;
            }
            if diff == 3 {
                has_three = true;
            }
            i = j;
        }

        if has_two { two_count += 1; }
        if has_three { three_count += 1; }
   }

    println!("{} lines have doubles", two_count);
    println!("{} lines have triples", three_count);
    println!("checksum: {}", two_count * three_count);
}
