use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let points: Vec<_> = BufReader::new(File::open("input/day09.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut components = line.split(',');
            Point {
                x: components.next().unwrap().parse().unwrap(),
                y: components.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut largest = 0;
    let mut width = 0;
    let mut height = 0;
    for (i, &a) in points.iter().enumerate() {
        for &b in points.iter().skip(i + 1) {
            let area = ((b.x - a.x).abs() + 1) * ((b.y - a.y).abs() + 1);
            if area > largest {
                let left = a.x.min(b.x);
                let right = a.x.max(b.x);
                let top = a.y.min(b.y);
                let bottom = a.y.max(b.y);
                let left_top = Point { x: left, y: top };
                let right_top = Point { x: right, y: top };
                let left_bottom = Point { x: left, y: bottom };
                let right_bottom = Point {
                    x: right,
                    y: bottom,
                };
                width = width.max(right);
                height = height.max(bottom);

                if is_inside(&points, left_top)
                    && is_inside(&points, right_top)
                    && is_inside(&points, right_bottom)
                    && is_inside(&points, left_bottom)
                    && !crosses_line(&points, left_top, right_top)
                    && !crosses_line(&points, right_top, right_bottom)
                    && !crosses_line(&points, right_bottom, left_bottom)
                    && !crosses_line(&points, left_bottom, left_top)
                {
                    largest = area;
                }
            }
        }
    }

    println!("Largest rectangle: {largest}");
}

fn is_inside(points: &[Point], test: Point) -> bool {
    let mut inside = 0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();

        if points[i].x == points[j].x {
            let up = points[i].y > points[j].y;
            if test.y < points[i].y && test.y < points[j].y
                || test.y > points[i].y && test.y > points[j].y
                || up && test.x < points[i].x
                || !up && test.x <= points[i].x
            {
                continue;
            }

            inside += if up { 1 } else { -1 };
        }
    }
    inside >= 1
}

fn crosses_line(points: &[Point], a: Point, b: Point) -> bool {
    let vertical = a.x == b.x;

    for i in 0..points.len() {
        let j = (i + 1) % points.len();

        if points[i].x == points[j].x && !vertical {
            let x = points[i].x;
            let y = a.y;
            let top = points[i].y.min(points[j].y);
            let bottom = points[i].y.max(points[j].y);
            if x <= a.x && x <= b.x || x >= a.x && x >= b.x || y <= top || y >= bottom {
                continue;
            }
            return true;
        } else if points[i].y == points[j].y && vertical {
            let y = points[i].y;
            let x = a.x;
            let left = points[i].x.min(points[j].x);
            let right = points[i].x.max(points[j].x);
            if y <= a.y && y <= b.y || y >= a.y && y >= b.y || x <= left || x >= right {
                continue;
            }
            return true;
        }
    }
    false
}
