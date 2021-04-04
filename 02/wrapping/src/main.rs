use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::convert::TryFrom;

fn main() -> io::Result<()> {
    let mut present_sizes : Vec<String> = Vec::new();
    let mut surface_areas : Vec<i32> = Vec::new();
    let mut slacks : Vec<i32> = Vec::new();
    let mut input = File::open("INPUT")?;
    let reader = BufReader::new(input);

    'lines: for line in reader.lines() {
        for tok in line.unwrap().split_whitespace() {
            match tok {
                "done" => break 'lines,
                _ => {
                    present_sizes.push(tok.to_string())
                }
            }
        }
    };

    let mut sum : i32 = 0;
    for i in present_sizes {
        let split = i.split('x');
        let dimensions : Vec<&str> = split.collect();
        let x : i32 = dimensions[0].parse().unwrap();
        let y : i32 = dimensions[1].parse().unwrap();
        let z : i32 = dimensions[2].parse().unwrap();

        let mut slack : i32 = 0;
        let surface_area : i32 = (2 * x * y) + (2 * x * z) + (2 * y * z);


        if ((x * y) < (x * z)) && ((x * y) < (y * z)) {
            slack = x * y;
        } else if ((x * z) < (y * z)) {
            slack = x * z;
        } else {
            slack = y * z;
        }

        surface_areas.push(surface_area);
        slacks.push(slack);
        sum += surface_area + slack;
    }

    println!("SA: {}", surface_areas[0]);
    println!("slack: {}", slacks[0]);
    println!("sum: {}", sum);

    Ok(())
}
