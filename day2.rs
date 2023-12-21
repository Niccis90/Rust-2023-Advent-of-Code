use std::time::Instant;

mod data;

fn part1(data: &str) -> usize {
    let mut games_possible: Vec<usize> = Vec::new();
    for (game, line) in data.lines().enumerate() {
        let (_, new_line) = line.split_once(":").unwrap();
        let sets: Vec<&str> = new_line.splitn(100, ";").collect();
        //println!("{:?}",sets);
        let mut checking: Vec<[u8; 3]> = Vec::new(); // index: 0 red , index: 1 green , index 2 blue ( sub vector logic )
        for set in sets.iter() {
            let mut color_count: [u8; 3] = [0; 3];
            let subsets: Vec<&str> = set.splitn(5, ",").collect();
            for subset in subsets.iter() {
                if subset.ends_with("red") {
                    let trimmed = subset.trim();
                    let temp: Vec<&str> = trimmed.split_whitespace().collect();
                    let number: u8 = temp[0].parse().unwrap();
                    //println!("{:?}",number);
                    color_count[0] = number;
                }
                if subset.ends_with("green") {
                    let trimmed = subset.trim();
                    let temp: Vec<&str> = trimmed.split_whitespace().collect();
                    let number: u8 = temp[0].parse().unwrap();
                    //println!("{:?}",number);
                    color_count[1] = number;
                }
                if subset.ends_with("blue") {
                    let trimmed = subset.trim();
                    let temp: Vec<&str> = trimmed.split_whitespace().collect();
                    let number: u8 = temp[0].parse().unwrap();
                    //println!("{:?}",number);
                    color_count[2] = number;
                }
            }
            //println!("{:?}",color_count);
            checking.push(color_count);
        }
        //println!("{:?}",checking);
        let mut condition: bool = false; // index: 0 red , index: 1 green , index 2 blue ( sub vector logic )
        let limits: [u8; 3] = [12, 13, 14];
        for showed in checking.iter() {
            for (idx, num) in showed.iter().enumerate() {
                if num > &limits[idx] {
                    condition = true;
                }
            }
        }
        if condition == false {
            games_possible.push(game + 1);
        }
    }
    let sum = games_possible.iter().sum();
    sum
}

fn part2(data: &str) -> u32 {
    let mut powers: Vec<u32> = Vec::new();
    for (game, line) in data.lines().enumerate() {
        let (_, new_line) = line.split_once(":").unwrap();
        let sets: Vec<&str> = new_line.splitn(100, ";").collect();
        //println!("{:?}",sets);
        let mut checking: Vec<[u32; 3]> = Vec::new(); // index: 0 red , index: 1 green , index 2 blue ( sub vector logic )
        for set in sets.iter() {
            let mut color_count: [u32; 3] = [0; 3];
            let subsets: Vec<&str> = set.splitn(5, ",").collect();
            for subset in subsets.iter() {
                if subset.ends_with("red") {
                    let trimmed = subset.trim();
                    let temp: Vec<&str> = trimmed.split_whitespace().collect();
                    let number: u32 = temp[0].parse().unwrap();
                    //println!("{:?}",number);
                    color_count[0] = number;
                }
                if subset.ends_with("green") {
                    let trimmed = subset.trim();
                    let temp: Vec<&str> = trimmed.split_whitespace().collect();
                    let number: u32 = temp[0].parse().unwrap();
                    //println!("{:?}",number);
                    color_count[1] = number;
                }
                if subset.ends_with("blue") {
                    let trimmed = subset.trim();
                    let temp: Vec<&str> = trimmed.split_whitespace().collect();
                    let number: u32 = temp[0].parse().unwrap();
                    //println!("{:?}",number);
                    color_count[2] = number;
                }
            }
            //println!("{:?}",color_count);
            checking.push(color_count);
        }
        //println!("{:?}", checking);
        let red_max = checking
            .iter()
            .map(|x| x[0])
            .map(|x| if x == 0 { 1 } else { x })
            .max()
            .unwrap();

        let green_max = checking
            .iter()
            .map(|x| x[1])
            .map(|x| if x == 0 { 1 } else { x })
            .max()
            .unwrap();

        let blue_max = checking
            .iter()
            .map(|x| x[2])
            .map(|x| if x == 0 { 1 } else { x })
            .max()
            .unwrap();

        let power = red_max * green_max * blue_max;
        //println!("{:?} {:?} {:?}", red_max, green_max, blue_max);
        powers.push(power);
    }

    let sum = powers.iter().sum();
    sum
}

fn main() {
    let time = Instant::now();
    let data: &str = data::DATA;
    let test: &str = data::TEST;
    //let part1 = part1(data);
    let part2 = part2(data);
    //println!("{:?}", part1);
    println!("{:?}", part2);

    println!("{:?}", time.elapsed());
}
