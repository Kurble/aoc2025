use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines: Vec<Vec<char>> = BufReader::new(File::open("input/day06.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let len = lines.len();

    let mut ops = Vec::<(char, usize, usize)>::new();

    let mut w = 0;
    for (i, &c) in lines[len - 1].iter().enumerate() {
        if c != ' ' {
            if !ops.is_empty() {
                ops.last_mut().unwrap().2 = w;
            }
            ops.push((c, i, 0));
            w = 0;
        } else {
            w += 1;
        }
    }
    ops.last_mut().unwrap().2 = w + 1;

    let mut result = 0u64;

    for (op, i, w) in ops {
        let mut output = if op == '*' { 1 } else { 0 };
        for i in i..i + w {
            let num = lines[..len - 1]
                .into_iter()
                .map(|line| line[i])
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .unwrap();
            if op == '*' {
                output *= num;
            } else {
                output += num;
            }
        }
        result += output;
    }

    println!("Total: {result}");
}
