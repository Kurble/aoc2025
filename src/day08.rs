use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::AddAssign,
};

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,

    island: u64,
}

fn main() {
    let mut coords: Vec<Coord> = BufReader::new(File::open("input/day08.txt").unwrap())
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.unwrap();
            let mut components = line.split(',');
            Coord {
                x: components.next().unwrap().parse().unwrap(),
                y: components.next().unwrap().parse().unwrap(),
                z: components.next().unwrap().parse().unwrap(),
                island: i as u64,
            }
        })
        .collect();

    let mut pairs: Vec<_> = coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            coords
                .iter()
                .skip(i + 1)
                .enumerate()
                .map(move |(j, b)| (i, i + 1 + j, a.dist_sq(b)))
        })
        .collect();

    pairs.sort_by_key(|&(_, _, d)| d);

    let mut islands_left = coords.len();
    // let mut connections = 0;
    let mut i = 0;
    while islands_left > 1 {
        let (a, b, d) = pairs[i];
        if d == -1 {
            i += 1;
            continue;
        }

        let island_a = coords[a].island;
        let island_b = coords[b].island;

        if island_a != island_b {
            islands_left -= 1;
            for coord in &mut coords {
                if coord.island == island_a {
                    coord.island = island_b;
                }
            }

            if islands_left == 1 {
                println!("Answer: {}", coords[a].x * coords[b].x);
            }
        }

        // connections += 1;
        pairs[i].2 = -1;
        i += 1;
    }

    let mut island_size = HashMap::<u64, usize>::new();
    for coord in &coords {
        island_size.entry(coord.island).or_default().add_assign(1);
    }

    let mut sizes: Vec<_> = island_size.into_values().collect();
    sizes.sort();
    sizes.reverse();

    println!("Islands: {sizes:?}");
    // println!("Answer: {}", sizes[0] * sizes[1] * sizes[2]);
}

impl Coord {
    fn dist_sq(&self, other: &Coord) -> i64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        dx * dx + dy * dy + dz * dz
    }
}
