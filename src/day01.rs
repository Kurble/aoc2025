use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = BufReader::new(File::open("input/day01.txt").unwrap());

    let mut state = 50;
    let mut zeroes = 0;

    let mut lines = input.lines();
    while let Some(Ok(line)) = lines.next() {
        match &line[..1] {
            "L" => {
                let left = line[1..].parse::<i32>().unwrap();
                for _ in 0..left {
                    state -= 1;
                    if state == -1 {
                        state = 99;
                    }
                    if state == 0 {
                        zeroes += 1;
                    }
                }
            }
            "R" => {
                let right = line[1..].parse::<i32>().unwrap();
                for _ in 0..right {
                    state += 1;
                    if state == 100 {
                        state = 0;
                        zeroes += 1;
                    }
                }
            }
            _ => panic!("Invalid input"),
        }
    }

    println!("Password: {zeroes}");
}
