use std::time::Instant;

mod data;

fn parser(data: &str) -> [u16; 1000] {
    let mut digits: [u16; 1000] = [0; 1000];
    for (index, line) in data.lines().enumerate() {
        let mut temp: Vec<char> = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                temp.push(char);
            }
        }
        let concat = format!("{}{}", temp.first().unwrap(), temp.last().unwrap());
        //println!("{:?}",concat);
        let number: u16 = concat.parse().unwrap();
        digits[index] = number
    }
    digits
}

fn part1(data: &str) -> u16 {
    let data = parser(data);
    let mut sum = 0;
    for i in data {
        sum += i;
    }
    sum as u16
}

fn part2(data: &str) -> u32 {
    let letters = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    data.lines()
        .map(|line| {
            let mut digit_iter = line.chars().enumerate().filter_map(|(i, ch)| {
                if let Some(digit) = ch.to_digit(10) {
                    Some(digit)
                } else {
                    let sub_str = &line[i..];
                    letters
                        .iter()
                        .enumerate()
                        .filter_map(|(digit, digit_char)| {
                            sub_str.starts_with(digit_char).then_some(digit as u32 + 1)
                        })
                        .next()
                }
            });
            let first = digit_iter.next().expect("digit not found");
            let last = digit_iter.last().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>()
}

fn main() {
    let time = Instant::now();
    let data: &str = data::DATA;
    let test: &str = data::TEST;
    let test2: &str = data::TEST2;
    //let parsed = parser(test);
    //println!("{:?}",parsed);
    //let part1 = part1(data);
    //println!("{:?}",part1);
    let part2 = part2(data);
    println!("{:?}", part2);
    println!("{:?}", time.elapsed());
}
