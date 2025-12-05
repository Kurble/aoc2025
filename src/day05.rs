use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut lines = BufReader::new(File::open("input/day05.txt").unwrap()).lines();

    let mut ranges = Vec::<(u64, u64)>::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (first, last) = line.split_once('-').unwrap();
        ranges.push((first.parse().unwrap(), last.parse().unwrap()));
    }

    let mut fresh = 0;
    while let Some(Ok(line)) = lines.next() {
        let id: u64 = line.parse().unwrap();
        for &(first, last) in &ranges {
            if id >= first && id <= last {
                fresh += 1;
                break;
            }
        }
    }

    println!("Fresh ids: {fresh}");

    ranges.sort_by_key(|&(k, _)| k);
    let mut counted_until = 0;
    let mut total = 0;
    for (first, last) in ranges {
        let first = first.max(counted_until + 1);
        if first <= last {
            total += (last - first) + 1;
            counted_until = last;
        }
    }

    println!("Total fresh ids: {total}");
}
