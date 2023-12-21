use std::{io::empty, process::Termination, time::Instant};

mod data;

// parse data
// get coordinates of stars
// scale start coords according to num empty cols and rows above and to the left of start X
// calculate the sum of each manhattan distance and sum
#[derive(Debug)]
struct universe {
    space: Vec<Vec<u16>>,
    coordinates_of_stars: Vec<[u32; 2]>,
    empty_row: Vec<u32>,
    empty_col: Vec<u32>,
}

impl universe {
    fn new() -> universe {
        universe {
            space: Vec::new(),
            coordinates_of_stars: Vec::new(),
            empty_row: Vec::new(),
            empty_col: Vec::new(),
        }
    }
    fn parse(&mut self, data: &str) {
        let mut hashtag_count = 1 as u16;
        for line in data.lines() {
            let mut temp = Vec::new();
            for char in line.chars() {
                if char == '#' {
                    temp.push(hashtag_count);
                    hashtag_count += 1;
                } else {
                    temp.push(0)
                }
            }
            self.space.push(temp);
        }
    }
    fn get_coords(&mut self) {
        for (idx1, row) in self.space.iter().enumerate() {
            for (idx2, col) in self.space[idx1].iter().enumerate() {
                if self.space[idx1][idx2] != 0 {
                    self.coordinates_of_stars.push([idx1 as u32, idx2 as u32])
                }
            }
        }
    }
    fn get_empty_row_col(&mut self) {
        for col in 0..self.space[0].len() {
            let mut temp = Vec::new();
            for row in 0..self.space.len() {
                temp.push(self.space[row][col])
            }
            if temp.iter().all(|x| *x == 0) {
                self.empty_col.push(col as u32)
            }
        }
        for row in 0..self.space.len() {
            let mut temp = Vec::new();
            for col in 0..self.space[0].len() {
                temp.push(self.space[row][col])
            }
            if temp.iter().all(|x| *x == 0) {
                self.empty_row.push(row as u32)
            }
        }
    }
    fn expand_universe(&mut self, multiplier: u32) {
        for coord in &mut self.coordinates_of_stars {
            let count_up = self.empty_row.iter().filter(|x| *x < &coord[0]).count();
            let count_left = self.empty_col.iter().filter(|x| *x < &coord[1]).count();

            coord[0] += count_up as u32 * (multiplier - 1);
            coord[1] += count_left as u32 * (multiplier - 1);
        }
    }
    fn manhattan_dist(&self, coord1: [u32; 2], coord2: [u32; 2]) -> u64 {
        let icord1 = [coord1[0] as i32, coord1[1] as i32];
        let icord2 = [coord2[0] as i32, coord2[1] as i32];

        ((icord2[0] - icord1[0]).abs() + (icord2[1] - icord1[1]).abs()) as u64
    }
    fn part1_2(&self) -> u64 {
        let mut sum = 0;
        for star in 0..self.coordinates_of_stars.len() {
            for star2 in star + 1..self.coordinates_of_stars.len() {
                sum += self.manhattan_dist(
                    self.coordinates_of_stars[star],
                    self.coordinates_of_stars[star2],
                );
            }
        }
        println!("{sum:?}");
        sum
    }
}

fn main() {
    let time = Instant::now();
    let test1 = data::TEST1;
    let test2 = data::TEST12;
    let data = data::DATA;
    let mut universe = universe::new();
    universe.parse(data);
    universe.get_coords();
    universe.get_empty_row_col();
    universe.expand_universe(1000000); // set multiplier to 2 for part1 and 1M for part2
    universe.part1_2();

    println!("{:?}", time.elapsed());
}
