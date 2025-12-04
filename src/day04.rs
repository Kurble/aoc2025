use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut cells: Vec<Vec<u8>> = BufReader::new(File::open("input/day04.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().trim().as_bytes().to_vec())
        .collect();

    let width = cells[0].len();
    let height = cells.len();

    let mut result = 0;
    let mut previous = result;

    loop {
        for y in 0..height {
            let above = if y > 0 { y - 1 } else { y };
            let below = if y < height - 1 { y + 1 } else { y };
            for x in 0..width {
                if cells[y][x] != b'@' {
                    continue;
                }

                let left = if x > 0 { x - 1 } else { x };
                let right = if x < width - 1 { x + 1 } else { x };

                let mut count = 0;
                for cy in above..=below {
                    for cx in left..=right {
                        if cy != y || cx != x {
                            if cells[cy][cx] == b'@' {
                                count += 1;
                            }
                        }
                    }
                }

                if count < 4 {
                    result += 1;
                    cells[y][x] = b'x';
                }
            }
        }

        if result == previous {
            break;
        } else {
            previous = result;
        }
    }

    println!("{result}");
}
