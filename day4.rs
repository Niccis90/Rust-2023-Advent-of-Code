use std::time::Instant;

mod data;

fn parse_input(data: &str) -> Vec<([u32; 10], [u32; 25])> {
    data.lines()
        .filter_map(|line| {
            let mut parts = line.split(":");
            parts.next();

            parts.next().and_then(|right| {
                let mut numbers = right.split("|");
                let winning_nums = numbers.next().unwrap();
                let your_nums = numbers.next().unwrap();
                let mut winning_num_iter = winning_nums
                    .split_whitespace()
                    .filter_map(|num| num.parse().ok());
                let mut winning_num_array = [0; 10];
                for num in winning_num_array.iter_mut() {
                    *num = winning_num_iter.next().unwrap();
                }
                let mut your_num_iter = your_nums
                    .split_whitespace()
                    .filter_map(|num| num.parse().ok());
                let mut your_num_array = [0; 25];
                for num in your_num_array.iter_mut() {
                    *num = your_num_iter.next().unwrap();
                }

                Some((winning_num_array, your_num_array))
            })
        })
        .collect()
}

/*
fn part1(data: &str) -> u32 {
    let data: Vec<([u32; 10], [u32; 25])> = parse_input(data);
    let mut sum: u32 = 0;
    let points: [u32; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]; // redo this and make " n * r ^ (n-1) ";
    for (winning_nums, your_nums) in data.iter() {
        let mut matches: u32 = 0;
        for i in winning_nums.iter() {
            for j in your_nums.iter() {
                if i == j {
                    matches += 1;
                }
            }
        }
        if matches == 1 {
            sum += points[0];
        } else if matches == 2 {
            sum += points[1];
        } else if matches == 3 {
            sum += points[2];
        } else if matches == 4 {
            sum += points[3];
        } else if matches == 5 {
            sum += points[4];
        } else if matches == 6 {
            sum += points[5];
        } else if matches == 7 {
            sum += points[6];
        } else if matches == 8 {
            sum += points[7];
        } else if matches == 9 {
            sum += points[8];
        } else if matches == 10 {
            sum += points[9];
        }
    }

    sum
}
*/

fn part2(data: &str) -> u32 {
    let data: Vec<([u32; 10], [u32; 25])> = parse_input(data);
    let mut number_of_cards: [u32; 204] = [1; 204];
    for (index, (winning_nums, your_nums)) in data.iter().enumerate() {
        let mut matches: u32 = 0;
        for i in winning_nums.iter() {
            for j in your_nums.iter() {
                if i == j {
                    matches += 1;
                }
            }
        }
        for i in 1..=matches {
            number_of_cards[index + i as usize] += number_of_cards[index]
        }
    }
    let sum = number_of_cards.iter().sum();
    sum
}

fn main() {
    let time = Instant::now();
    let data: &str = data::DATA;
    let test: &str = data::TEST;
    //let part1 = part1(data);
    let part2 = part2(data);
    //println!("{}", part1);
    println!("{}", part2);
    println!("{:?}", time.elapsed());
}
