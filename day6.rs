use std::time::Instant;

mod data;

fn parser(data: &str) -> ([u64; 4], [u64; 4]) {
    let mut time = [0; 4];
    let mut dist = [0; 4];
    for line in data.lines() {
        if line.starts_with("Time:") {
            let mut splitted = line.split(":");
            splitted.next();
            let vals = splitted.next().unwrap();
            let mut vals_iter = vals
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            for num in time.iter_mut() {
                *num = vals_iter.next().unwrap();
            }
        } else if line.starts_with("Distance") {
            let mut splitted = line.split(":");
            splitted.next();
            let vals = splitted.next().unwrap();
            let mut vals_iter = vals
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());
            for num in dist.iter_mut() {
                *num = vals_iter.next().unwrap();
            }
        }
    }
    (time, dist)
}

fn part1(data: &str, num_of_games: u8) -> u64 {
    let (time, dist) = parser(data);
    let mut times: u64 = 1;
    for game in 0..num_of_games {
        let dist_to_beat = dist[game as usize];
        let total_time = time[game as usize];
        let mut sum = 0;
        for i in 0..=total_time {
            if i * (total_time - i) > dist_to_beat {
                sum += 1;
            }
        }
        times *= sum;
    }

    times
}

fn parser2(data: &str) -> (u64, u64) {
    let mut time = 0;
    let mut dist = 0;
    for line in data.lines() {
        if line.starts_with("Time:") {
            let mut splitted = line.split(":");
            splitted.next();
            let vals = splitted.next().unwrap();
            let concat = vals.replace(" ", "");
            time = concat.parse().unwrap();
        } else if line.starts_with("Distance") {
            let mut splitted = line.split(":");
            splitted.next();
            let vals = splitted.next().unwrap();
            let concat = vals.replace(" ", "");
            dist = concat.parse().unwrap();
        }
    }
    (time, dist)
}

fn part2(data: &str) -> u64 {
    let (time, dist) = parser2(data);
    let mut times: u64 = 1;
    let dist_to_beat = dist;
    let total_time = time;
    let mut sum = 0;
    for i in 0..=total_time {
        if i * (total_time - i) > dist_to_beat {
            sum += 1;
        }
    }
    times *= sum;

    times
}

fn main() {
    let time = Instant::now();
    let data: &str = data::DATA;
    let test: &str = data::TEST;
    //let part1 = part1(data, 4);
    let part2 = part2(data);
    //println!("{:?}", part1);
    println!("{:?}", part2);
    println!("{:?}", time.elapsed());
}
