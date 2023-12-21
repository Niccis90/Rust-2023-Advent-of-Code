use std::{
    collections::{HashMap, HashSet},
    path::MAIN_SEPARATOR,
    time::Instant,
};
mod data;

#[derive(Debug, Clone)]
struct Pipeline {
    pipes: Vec<Vec<pipe>>,
    start: (usize, usize),
    previous_position: (usize, usize),
}
#[derive(Debug, Clone, Copy)]

struct pipe {
    S: bool,
    south: bool,
    north: bool,
    west: bool,
    east: bool,
}

impl pipe {
    fn new() -> pipe {
        pipe {
            S: false,
            south: false,
            north: false,
            west: false,
            east: false,
        }
    }
    fn determine_dir(mut self, sign: char) -> pipe {
        match sign {
            '|' => {
                self.north = true;
                self.south = true;
                self.S = false
            }
            '-' => {
                self.east = true;
                self.west = true;
                self.S = false
            }
            'L' => {
                self.north = true;
                self.east = true;
                self.S = false
            }
            'J' => {
                self.north = true;
                self.west = true;
                self.S = false
            }
            '7' => {
                self.south = true;
                self.west = true;
                self.S = false
            }
            'F' => {
                self.south = true;
                self.east = true;
                self.S = false
            }
            '.' => {
                self.south = false;
                self.north = false;
                self.west = false;
                self.east = false;
                self.S = false
            }
            'S' => {
                self.S = true;
                self.south = true;
                self.north = true;
                self.west = true;
                self.east = true
            }
            _ => eprintln!("error, random char found"),
        }
        self
    }
}

impl Pipeline {
    fn new() -> Pipeline {
        Pipeline {
            pipes: Vec::new(),
            start: (0, 0),
            previous_position: (0, 0),
        }
    }

    fn parse(&mut self, data: &str) {
        for line in data.lines() {
            let vec = line
                .chars()
                .map(|char| pipe::new().determine_dir(char))
                .collect::<Vec<pipe>>();
            self.pipes.push(vec)
        }
        self.find_start();
    }
    fn find_start(&mut self) {
        for (idx, i) in self.pipes.iter().enumerate() {
            for (idx2, j) in self.pipes[idx].iter().enumerate() {
                if j.S == true
                    && j.south == true
                    && j.north == true
                    && j.west == true
                    && j.east == true
                {
                    self.start = (idx, idx2)
                }
            }
        }
    }
    fn get_pipe(&self, cords: (i64, i64)) -> pipe {
        self.pipes[cords.0 as usize][cords.1 as usize]
    }
    fn is_possible_move(&self, dir: (i64, i64), pos: (usize, usize)) -> bool {
        let destination = (pos.0 as i64 + dir.0, pos.1 as i64 + dir.1);
        let ipos = (pos.0 as i64, pos.1 as i64);
        let previous_position_i64 = (
            self.previous_position.0 as i64,
            self.previous_position.1 as i64,
        );
        if destination == previous_position_i64
            || destination.0 < 0
            || destination.1 < 0
            || destination.1 >= self.pipes[0].len() as i64
            || destination.0 >= self.pipes.len() as i64
        {
            return false;
        }

        if dir == (1, 0) {
            if self.get_pipe(destination).north == true && self.get_pipe(ipos).south == true {
                return true;
            } else {
                return false;
            }
        } else if dir == (-1, 0) {
            if self.get_pipe(destination).south == true && self.get_pipe(ipos).north == true {
                return true;
            } else {
                return false;
            }
        } else if dir == (0, 1) {
            if self.get_pipe(destination).west == true && self.get_pipe(ipos).east == true {
                return true;
            } else {
                return false;
            }
        } else {
            if self.get_pipe(destination).east == true && self.get_pipe(ipos).west == true {
                return true;
            } else {
                return false;
            }
        }
    }
    fn move_to(&self, pos: (usize, usize)) -> (usize, usize) {
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for direction in dirs {
            let destination = (pos.0 as i64 + direction.0, pos.1 as i64 + direction.1);
            if self.is_possible_move(direction, pos) {
                return (destination.0 as usize, destination.1 as usize);
            }
        }
        eprintln!("nothing found for pos : {pos:?} {:?}", self.start);
        std::process::exit(0);
    }
    fn navigate_pipes(&mut self) -> (u128, Vec<(i64, i64)>) {
        let mut steps = 0;

        let mut moves_to_make = Vec::new();
        let mut visited = Vec::new();

        visited.push((self.start.0 as i64, self.start.1 as i64));

        moves_to_make.push(self.move_to(self.start));

        while let Some(loc) = moves_to_make.pop() {
            if loc == self.start && steps > 0 {
                steps += 1;

                break;
            }
            visited.push((loc.0 as i64, loc.1 as i64));
            let next_pos = self.move_to(loc);

            moves_to_make.push(next_pos);
            self.previous_position = loc;
            steps += 1;
        }

        (steps, visited) // divide steps by 2 for part1 answer
    }
    fn shoelace_area_calc(&self, verticies: &Vec<(i64, i64)>) -> u128 {
        let length = verticies.len();

        if length < 3 {
            println!("{verticies:?} is not a polygon");
            return 0;
        }

        let mut sum1: i64 = 0;
        let mut sum2: i64 = 0;

        for i in 0..length - 1 {
            sum1 = sum1 + verticies[i].0 * verticies[i + 1].1;
            sum2 = sum2 + verticies[i].1 * verticies[i + 1].0;
        }

        // for last vertex for last connecting to the first.

        sum1 = sum1 + verticies[length - 1].0 * verticies[0].1;
        sum2 = sum2 + verticies[length - 1].1 * verticies[0].0;

        let area = (sum1 - sum2).abs() / 2;
        area as u128
    }

    fn part2(&mut self) -> u128 {
        let (pipe_len, pipe_cords) = self.navigate_pipes();

        let area = self.shoelace_area_calc(&pipe_cords);

        let inside_points = area + 1 - (pipe_len / 2);

        println!("{inside_points:?}");

        inside_points
    }
}

fn main() {
    let time = Instant::now();
    let test1 = data::TEST1;
    let test2 = data::TEST2;
    let test3 = data::TEST3;

    let data = data::DATA;
    let mut map = Pipeline::new();
    map.parse(data);
    let part1 = map.navigate_pipes();
    let part2 = map.part2();
    println!("{part2:?}");

    println!("{:?}", time.elapsed());
}
