use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut val = 0;

    let mut val_history = Vec::new();
    let mut mutations = Vec::new();
    for line in str.lines() {
        if line.is_empty() {
            continue;
        }

        let amount = line.parse().unwrap();
        mutations.push(amount);
    }
    val_history.push(val);
    loop {
        for amount in &mutations {
            val += amount;

            if val_history.contains(&val) {
                println!("Reached frequency {:?} twice", val);
                return;
            }
            val_history.push(val);
        }
    }
}
