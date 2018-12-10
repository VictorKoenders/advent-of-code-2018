use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    const DIFF: u8 = b'a' - b'A';
    let mut lowest = (9999, 'a');

    for c in b'a' ..= b'z' {
        let mut bytes = str.trim().as_bytes().to_vec();
        bytes.retain(|b| *b != c && *b != c - DIFF);

        let mut i = 1;

        while i < bytes.len() {
            if i == 0 {
                i += 1;
                continue;
            }
            if bytes[i] + DIFF == bytes[i-1] || bytes[i] - DIFF == bytes[i-1] {
                i -= 1;
                bytes.remove(i);
                bytes.remove(i);
            } else {
                i += 1;
            }
        }
        // println!("{:?}", std::str::from_utf8(&bytes));
        println!("removing {} gives {} units", c as char, bytes.len());
        if bytes.len() < lowest.0 {
            lowest = ( bytes.len(), c as char );
        }
    }

    println!("Lowest is char {} with {} units", lowest.1, lowest.0);
}
