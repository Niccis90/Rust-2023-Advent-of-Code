use core::num;
use std::{time::Instant, vec};

mod data;

fn part1(data: &str) -> u64 {
    let mut matrix: [[u64; 20]; 8] = [[0; 20]; 8];
    let mut first_line = false;
    let mut matrix_location = 0;
    for line in data.lines() {
        if line.starts_with("seeds:") && first_line == false {
            let mut split = line.split(":");
            split.next();
            let numbers = split.next().unwrap();
            let mut numbers_iter = numbers
                .split_whitespace()
                .filter_map(|num| num.parse().ok());
            for num in matrix[0].iter_mut() {
                *num = numbers_iter.next().unwrap();
                first_line = true;
            }
        } else if line.is_empty() {
            continue;
        } else if !line.as_bytes()[0].is_ascii_digit() {
            matrix_location += 1;
        } else {
            let mut line_iter = line.split_whitespace().filter_map(|num| num.parse().ok());
            let mut contents: [u64; 3] = [0; 3]; // index0: destination range start , index1: source range start, index2: range length
            for num in contents.iter_mut() {
                *num = line_iter.next().unwrap();
            }

            let destination_start = contents[0];
            let source_start = contents[1];
            let range_len = contents[2];

            let mut changes = Vec::new();

            for (col_idx, source) in matrix[matrix_location - 1].iter().enumerate() {
                if *source >= source_start && *source < source_start + range_len {
                    let offset = *source - source_start;
                    let destination = destination_start + offset;
                    changes.push((matrix_location, col_idx, destination))
                }
            }
            for (loc, col, val) in changes {
                matrix[loc][col] = val
            }
            let mut post_changes = Vec::new();

            for (col_idx, item) in matrix[matrix_location].iter().enumerate() {
                if item == &0 {
                    post_changes.push((
                        matrix_location,
                        col_idx,
                        matrix[matrix_location - 1][col_idx]
                    ))
                }
            }
            for (loc, col, val) in post_changes {
                matrix[loc][col] = val;
            }
        }
        //println!("ok");
    }
    let smallest = *matrix[7].iter().min().unwrap();
    //println!("content {:?} ", matrix);
    smallest
}

fn main() {
    let time = Instant::now();
    let data = data::DATA;
    let test = data::TEST;
    let part1 = part1(data);
    //let part2 = part2(data);
    println!("{:?}", part1);
    //println!("{:?}", part2);
    println!("{:?}", time.elapsed());
}
