use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let manifold: Vec<_> = BufReader::new(File::open("input/day07.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    let x = manifold[0].iter().position(|&c| c == b'S').unwrap();

    let mut cache = HashMap::new();

    println!("{}", trace_beam(&manifold, &mut cache, x, 1));
}

fn trace_beam(
    manifold: &[Vec<u8>],
    cache: &mut HashMap<(usize, usize), usize>,
    x: usize,
    y: usize,
) -> usize {
    if y == manifold.len() - 1 {
        return 1;
    }

    if let Some(cached) = cache.get(&(x, y)) {
        return *cached;
    }

    let c = manifold[y][x];

    let result = if c == b'.' {
        trace_beam(manifold, cache, x, y + 1)
    } else if c == b'^' {
        trace_beam(manifold, cache, x - 1, y + 1) + trace_beam(manifold, cache, x + 1, y + 1)
    } else {
        0
    };

    cache.insert((x, y), result);

    result
}
