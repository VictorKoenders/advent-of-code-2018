use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Range;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let line: Vec<u8> = str.lines().next().unwrap().bytes().collect();
    let position_start = line.iter().position(|c| *c == b'<').unwrap();
    let position_comma = line.iter().position(|c| *c == b',').unwrap();
    let position_end = line.iter().position(|c| *c == b'>').unwrap();
    let velocity_start = line
        .iter()
        .skip(position_end + 1)
        .position(|c| *c == b'<')
        .unwrap()
        + position_end
        + 1;
    let velocity_comma = line
        .iter()
        .skip(position_end + 1)
        .position(|c| *c == b',')
        .unwrap()
        + position_end
        + 1;
    let velocity_end = line
        .iter()
        .skip(position_end + 1)
        .position(|c| *c == b'>')
        .unwrap()
        + position_end
        + 1;

    let position_x_range = (position_start + 1)..(position_comma);
    let position_y_range = (position_comma + 1)..(position_end);
    let velocity_x_range = (velocity_start + 1)..(velocity_comma);
    let velocity_y_range = (velocity_comma + 1)..(velocity_end);

    let mut particles = Vec::new();

    let mut bounds = Bounds::default();

    for line in str.lines() {
        let particle = Particle::from_line(
            line,
            position_x_range.clone(),
            position_y_range.clone(),
            velocity_x_range.clone(),
            velocity_y_range.clone(),
        );
        if bounds.min_x > particle.x { bounds.min_x = particle.x; }
        if bounds.min_y > particle.y { bounds.min_y = particle.y; }
        if bounds.max_x < particle.x { bounds.max_x = particle.x; }
        if bounds.max_y < particle.y { bounds.max_y = particle.y; }
        particles.push(particle);
    }

    println!("Bounds: {:?}", bounds);

    for i in 0..10 {
        println!("{} seconds", i);
        let mut file = File::create(&format!("{}.txt", i)).unwrap();
        let particle_positions: Vec<(usize, usize)> = particles.iter().filter_map(|p| bounds.to_grid_position((p.x, p.y))).collect();
        /*let mut grid = vec![vec![0; bounds.total_height()]; bounds.total_width()];
        for particle in &mut particles {
            if let Some((x, y)) = bounds.to_grid_position((particle.x, particle.y)) {
                grid[x][y] += 1;
            }

            particle.x += particle.velocity_x;
            particle.y += particle.velocity_y;
        }*/

        for y in 0..bounds.total_height() {
            let mut line = String::with_capacity(bounds.total_width());
            for x in 0..bounds.total_width() {
                let has_particle = particle_positions.iter().any(|(px, py)| *px == x && *py == y);
                line.push(if has_particle { '#' } else { '.' });
            }
            writeln!(file, "{}", line).unwrap();
        }

        for particle in &mut particles {
            particle.x += particle.velocity_x;
            particle.y += particle.velocity_y;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bounds {
    pub min_x: isize,
    pub min_y: isize,
    pub max_x: isize,
    pub max_y: isize,
}

impl Bounds {
    pub fn total_width(&self) -> usize { self.min_x.abs() as usize + self.max_x.abs() as usize + 1 }
    pub fn total_height(&self) -> usize { self.min_y.abs() as usize + self.max_y.abs() as usize + 1 }

    pub fn to_grid_position(&self, (x, y): (isize, isize)) -> Option<(usize, usize)> {
        if x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y {
            None
        } else {
            let x = x - self.min_x;
            let y = y - self.min_y;
            assert!(x >= 0);
            assert!(y >= 0);
            Some((x as usize, y as usize))
        }
    }
}

impl Default for Bounds {
    fn default() -> Bounds {
        Bounds {
            min_x: isize::max_value(),
            min_y: isize::max_value(),
            max_x: isize::min_value(),
            max_y: isize::min_value(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub x: isize,
    pub y: isize,
    pub velocity_x: isize,
    pub velocity_y: isize,
}

impl Particle {
    pub fn from_line(
        line: &str,
        position_x_range: Range<usize>,
        position_y_range: Range<usize>,
        velocity_x_range: Range<usize>,
        velocity_y_range: Range<usize>,
    ) -> Particle {
        let line = line.as_bytes();
        let x = std::str::from_utf8(&line[position_x_range])
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let y = std::str::from_utf8(&line[position_y_range])
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let velocity_x = std::str::from_utf8(&line[velocity_x_range])
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let velocity_y = std::str::from_utf8(&line[velocity_y_range])
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        Particle {
            x,
            y,
            velocity_x,
            velocity_y,
        }
    }
}
