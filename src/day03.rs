use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut banks = BufReader::new(File::open("input/day03.txt").unwrap()).lines();
    let mut result = 0;

    while let Some(Ok(bank)) = banks.next() {
        let digits = bank.trim().as_bytes();
        result += find_joltage_recursive(digits, 12);
    }

    println!("Total joltage: {result}");
}

fn find_joltage_recursive(bank: &[u8], mut depth: u32) -> u64 {
    if depth == 1 {
        let mut max = 0;
        for bat in bank {
            max = max.max((bat - b'0') as u64);
        }
        return max;
    }

    depth -= 1;

    let mut best_start = 0;
    let mut value_start = (bank[0] - b'0') as u64;
    for i in 1..bank.len() - depth as usize {
        let value = (bank[i] - b'0') as u64;
        if value > value_start {
            best_start = i;
            value_start = value;
        }
    }

    let front = value_start * 10u64.pow(depth);
    front + find_joltage_recursive(&bank[best_start + 1..], depth)
}
