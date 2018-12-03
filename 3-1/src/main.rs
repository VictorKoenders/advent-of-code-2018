use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut claims = Vec::new();

    for mut line in str.lines() {
        let space = line.bytes().position(|b| b == b' ').unwrap();
        let id = &line[1..space];
        line = &line[space+3..];
        let comma = line.bytes().position(|b| b == b',').unwrap();
        let x = line[..comma].parse().unwrap();
        line = &line[comma+1..];
        let colon = line.bytes().position(|b| b == b':').unwrap();
        let y = line[..colon].parse().unwrap();
        line = &line[colon+2..];
        let times = line.bytes().position(|b| b == b'x').unwrap();
        let w = line[..times].parse().unwrap();
        let h = line[times+1..].parse().unwrap();

        claims.push(Claim {
            id, x, y, w, h
        });
    }

    let mut area = [[0u8;1000];1000];

    for claim in claims {
        for x in claim.x .. claim.x + claim.w {
            for y in claim.y .. claim.y + claim.h {
                area[x as usize][y as usize] += 1;
            }
        }
    }

    let mut overlap_count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if area[x][y] > 1 {
                overlap_count += 1;
            }
        }
    }

    println!("{:?} tiles overlap", overlap_count);
}

struct Claim<'a> {
    pub id: &'a str,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}
