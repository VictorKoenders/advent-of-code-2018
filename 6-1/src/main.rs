#![feature(never_type)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut points = Vec::new();
    let mut max = (0, 0);
    let mut id = 1;
    for line in str.lines() {
        if line.trim().is_empty() { continue; }
        let mut point: Point = line.parse().unwrap();
        point.id = Id::Higher(id);
        id += 1;

        if point.x > max.0 { max.0 = point.x; }
        if point.y > max.1 { max.1 = point.y; }
        points.push(point);
    }
    let max = ((max.0 + 1) as usize, (max.1 + 1) as usize);

    let mut grid = vec![vec![Id::Unknown; max.1]; max.0];

    for x in 0..max.0 {
        for y in 0..max.1 {
            let mut nearest_distance = 999;
            let mut nearest = Vec::new();
            for point in &points {
                let distance = (x as isize - point.x as isize).abs() + (y as isize - point.y as isize).abs();
                if distance < nearest_distance {
                    nearest.clear();
                    nearest.push(point.id.clone());
                    nearest_distance = distance;
                } else if distance == nearest_distance {
                    nearest.push(point.id.clone());
                }
            }

            if nearest_distance == 0 {
                grid[x][y] = nearest[0];
            } else if nearest.len() == 1 {
                grid[x][y] = nearest[0].downgrade();
            } else {
                grid[x][y] = Id::Unknown;
            }
        }
    }

    for y in 0..max.1 {
        for x in 0..max.0 {
            match grid[x][y] {
                Id::Unknown => print!(" . "),
                Id::Lower(id) => print!("L{:02}", id),
                Id::Higher(id) => print!("H{:02}", id),
            }
        }
        println!();
    }

    let mut edges = Vec::new();
    for y in 0..max.1 {
        edges.push(grid[0][y]);
        edges.push(grid[max.0 - 1][y]);
    }
    for x in 0..max.0 {
        edges.push(grid[x][0]);
        edges.push(grid[x][max.1 - 1]);
    }

    'outer: for point in &points {
        for edge in &edges {
            if point.id.matches(&edge) {
                continue 'outer;
            }
        }
        println!("{:?} is not infinite", point.id);
        let mut area = 0;
        for x in 0..max.0 {
            for y in 0..max.1 {
                if grid[x][y].matches(&point.id) {
                    area+=1;
                }
            }
        }
        println!("Area is {:?}", area);
    }
}

#[derive(Debug, Clone)]
struct Point {
    pub id: Id,
    pub x: u32,
    pub y: u32,
}

impl Id {
    pub fn matches(&self, other: &Id) -> bool {
        match (self, other) {
            (Id::Higher(id1), Id::Lower(id2)) => id1 == id2,
            (Id::Lower(id1), Id::Higher(id2)) => id1 == id2,
            (Id::Higher(id1), Id::Higher(id2)) => id1 == id2,
            (Id::Lower(id1), Id::Lower(id2)) => id1 == id2,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Id {
    Unknown,
    Higher(u8),
    Lower(u8),
}

impl Id {
    pub fn downgrade(&self) -> Id {
        match self {
            Id::Unknown => Id::Unknown,
            Id::Higher(id) => Id::Lower(*id),
            Id::Lower(id) => Id::Lower(*id),
        }
    }
}

impl FromStr for Point {
    type Err = !;
    fn from_str(s: &str) -> Result<Point, !> {
        let mut split = s.split(',');
        let x = split.next().unwrap().trim().parse().unwrap();
        let y = split.next().unwrap().trim().parse().unwrap();
        Ok(Point { id: Id::Unknown, x, y })
    }
}

