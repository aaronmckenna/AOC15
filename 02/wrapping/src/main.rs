use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("INPUT")?;
    let present_sizes : Vec<String> = collect_sizes(input);

    //let sum : i32 = calc_sum(present_sizes);

    let ribbon : i32 = get_ribbon(present_sizes);

    println!("ribbon: {}", ribbon);

    Ok(())
}

fn collect_sizes(input : File) -> Vec<String> {
    let mut present_sizes : Vec<String> = Vec::new();
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

    present_sizes
}

fn calc_slack(x : i32, y : i32, z : i32) -> i32 {
    let slack;
    if ((x * y) < (x * z)) && ((x * y) < (y * z)) {
        slack = x * y;
    } else if (x * z) < (y * z) {
        slack = x * z;
    } else {
        slack = y * z;
    }
    slack
}

fn calc_sum(present_sizes : Vec<String>) -> i32 {
    let mut sum : i32 = 0;
    for size in present_sizes {
        let split = size.split('x');
        let dimensions : Vec<&str> = split.collect();
        let x : i32 = dimensions[0].parse().unwrap();
        let y : i32 = dimensions[1].parse().unwrap();
        let z : i32 = dimensions[2].parse().unwrap();
        let slack : i32 = calc_slack(x, y, z);
        let surface_area : i32 = (2 * x * y) + (2 * x * z) + (2 * y * z);

        sum += surface_area + slack;
    }
    sum
}

fn get_ribbon(present_sizes : Vec<String>) -> i32 {
    let mut sum = 0;
    for size in present_sizes {
        let split = size.split('x');
        let dimensions : Vec<&str> = split.collect();
        let x : i32 = dimensions[0].parse().unwrap();
        let y : i32 = dimensions[1].parse().unwrap();
        let z : i32 = dimensions[2].parse().unwrap();
        sum += calc_ribbon(x, y, z);
    }
    sum
}

fn calc_ribbon(x : i32, y : i32, z : i32) -> i32 {
    let bow = x * y * z;
    let ribbon;
    if (x + x + y + y) < (x + x + z + z) &&
        (x + x + y + y) < (y + y + z + z) {
        ribbon = x + x + y + y;
    } else if (x + x + z + z) < (y + y + z + z) {
        ribbon = x + x + z + z;
    } else {
        ribbon = y + y + z + z;
    }
    bow + ribbon
}
