use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let input = File::open("./src/input.txt").expect("cannot open file");
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let mut index: i32 = 0;

        for num in line.unwrap().split_whitespace() {
            let parsed_num = num.parse().unwrap();
            if index == 0 {
                left.push(parsed_num)
            } else {
                right.push(parsed_num)
            }
            index += 1;
        }
    }

    left.sort();
    right.sort();
    let mut total: i32 = 0;

    for (l_val, r_val) in left.iter().zip(right.iter()) {
        total += (l_val - r_val).abs();
    }

    println!("Result: {}", total)
}
