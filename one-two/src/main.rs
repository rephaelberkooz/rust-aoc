use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut num_map = HashMap::new();

    let input = File::open("./src/input.txt").expect("cannot open file");
    let reader = BufReader::new(input);

    // iterating over lines in this file
    for line in reader.lines() {
        let mut index: i32 = 0;

        // pushing left values into a vector
        // pushing right values in HashMap, incrementing new values
        for num in line.unwrap().split_whitespace() {
            let parsed_num = num.parse().unwrap();
            if index == 0 {
                left.push(parsed_num)
            } else {
                num_map
                    .entry(parsed_num)
                    .and_modify(|val| *val += 1)
                    .or_insert(1);
            }
            index += 1;
        }
    }

    let mut similarity_score: i32 = 0;

    for l_val in left {
        let map_val = num_map.get(&l_val).unwrap_or(&0);
        similarity_score += l_val * map_val;
    }

    println!("Similarity Score: {}", similarity_score)
}
