use std::{collections::HashMap, time::Instant};

mod data;
#[derive(Debug)]
struct Tree {
    map: HashMap<String, [String; 2]>,
    dirs: Vec<usize>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            map: HashMap::new(),
            dirs: Vec::new(),
        }
    }
    fn parse(&mut self, data: &str) {
        let mut splited = data.split("\n\n");

        self.dirs = splited
            .next()
            .unwrap()
            .chars()
            .map(|char| if char == 'L' { 0 } else { 1 })
            .collect();

        let other_data = splited.next().unwrap();

        for line in other_data.lines() {
            let mut split = line.split("=");
            let name = split.next().unwrap().trim().to_string();

            let paths = split.next().unwrap().trim();
            let mut paths_split = paths.split(",");
            let path_left = paths_split.next().unwrap().replace("(", "");
            let path_right = paths_split
                .next()
                .unwrap()
                .replace(")", "")
                .trim()
                .to_string();

            self.map.insert(name, [path_left, path_right]);
        }
    }
    fn get(&self, current: &String, count: usize) -> String {
        let val = self.map.get(current).unwrap()[self.dirs[count % self.dirs.len()]].clone();
        val
    }
    fn gcd(x: usize, y: usize) -> usize {
        if y == 0 {
            x
        } else {
            Self::gcd(y, x % y)
        }
    }
    fn lcm(x: usize, y: usize) -> usize {
        x * y / Self::gcd(x, y)
    }
    fn gcd_of(list: &[usize]) -> usize {
        let mut iter = list.iter();
        let first = *iter.next().unwrap();
        let second = *iter.next().unwrap();
        let mut ans = Self::gcd(first, second);
        while let Some(&next) = iter.next() {
            ans = Self::gcd(ans, next);
        }
        ans
    }
    fn lcm_of(list: &[usize]) -> usize {
        let mut iter = list.iter();
        let first = *iter.next().unwrap();
        let second = *iter.next().unwrap();
        let mut ans = Self::lcm(first, second);
        while let Some(&next) = iter.next() {
            ans = Self::lcm(ans, next);
        }
        ans
    }

    fn part1(&mut self, data: &str) -> u16 {
        let mut current = "AAA".to_string();
        let mut count = 0;

        while current != "ZZZ" {
            current = self.get(&current, count);
            count += 1;
        }

        count as u16
    }

    fn part2(&mut self, data: &str) -> u64 {
        let mut queue = self
            .map
            .keys()
            .filter(|key| key.ends_with("A"))
            .collect::<Vec<_>>();

        let mut ans = Vec::new();

        for entry in queue {
            let mut current = entry.clone();
            let mut count = 0;

            while !current.ends_with("Z") {
                current = self.get(&current, count);
                count += 1;
            }
            println!("{entry:?} -> {current:?} in {count} steps");
            ans.push(count)
        }

        //let gcm = Self::gcd_of(&ans);
        let answer = Self::lcm_of(&ans);

        answer as u64
    }
}

fn main() {
    let time = Instant::now();
    let data = data::DATA;
    //
    let mut tree = Tree::new();
    tree.parse(data);
    //
    let part1 = tree.part1(data);
    println!("{:?}", part1);
    //
    let part2 = tree.part2(data);
    println!("{:?}", part2);

    println!("{:?}", time.elapsed());
}
