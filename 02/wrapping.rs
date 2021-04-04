use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut present_sizes : Vec<str> = Vec::new();
    let mut surface_areas : Vec<i32> = Vec::new();
    let mut slacks : Vec<i32> = Vec::new();
    let mut input = File::create("TEST1")?;
    let reader = BufReader::new(input);

    'lines: for line in reader.lines() {
        for token in line.unwrap().split_whitespace() {
            match token {
                "done" => break 'lines,
                _ => present_sizes.push(token),
            };
        }
    }

    for i in present_sizes {
        let mut split = present_sizes[i].split('x');
        let x : i32 = split.parse().unwrap();
        let y : i32 = split.next().parse().unwrap();
        let z : i32 = split.next().parse().unwrap();
        let surface_area : i32 = (2 * x * y) + (2 * x * z) + (2 * y * z);
        let mut slack : i32 = 0;

        if (x * y) > (x * z) && (x * y) > (y * z) {
            slack = x * y;
        } else if (x * z) > (y * z) {
            slack = x * z;
        } else {
            slack = y * z;
        }
        surface_areas.push(surface_area);
        slacks.push(slack);
    }

    println!("SA: {}", surface_areas[0]);
    println!("slack: {}", slacks[0]);
}
