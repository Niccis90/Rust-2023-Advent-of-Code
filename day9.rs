use std::time::Instant;

mod data;

#[derive(Debug, Clone)]
struct oasis_data {
    data: Vec<sequences>,
}

impl oasis_data {
    fn new() -> oasis_data {
        oasis_data { data: Vec::new() }
    }
    fn fill_data(&mut self, data: &str, reverse: bool) {
        for line in data.lines() {
            let mut sequence = sequences::new();

            if reverse == true {
                for num in line.split_whitespace().rev() {
                    sequence.fill_input(num.parse().unwrap())
                }
            } else {
                for num in line.split_whitespace() {
                    sequence.fill_input(num.parse().unwrap())
                }
            }

            sequence.fill_sequence_ans();
            self.data.push(sequence.clone());
        }
    }
    fn part(&self) -> i32 {
        let mut sum = 0;
        for i in self.data.iter() {
            sum += i.ans.unwrap();
        }

        sum
    }
}

#[derive(Debug, Clone)]
struct sequences {
    input: Vec<i32>,
    sequence: Option<Vec<Vec<i32>>>,
    ans: Option<i32>,
}

impl sequences {
    fn new() -> sequences {
        sequences {
            input: Vec::new(),
            sequence: Some(Vec::new()),
            ans: Some(0),
        }
    }
    fn fill_input(&mut self, x: i32) {
        self.input.push(x)
    }
    fn fill_sequence_ans(&mut self) {
        match &mut self.sequence {
            Some(seq) => seq.push(self.input.clone()),
            None => {
                self.sequence = Some(vec![self.input.clone()]);
            }
        }
        if let Some(seq) = &mut self.sequence {
            while let Some(last) = seq.last() {
                if last.len() == 1 {
                    seq.push(vec![0, 0]);
                    break;
                }

                let mut next_layer = last.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();

                if next_layer.iter().all(|&x| x == 0) {
                    next_layer.push(0);
                    seq.push(next_layer);
                    break;
                }

                seq.push(next_layer);
            }
            for i in (1..seq.len() - 1).rev() {
                if let Some(next_layer_last) = seq[i - 1].last() {
                    if let Some(last) = seq[i].last() {
                        let val = last + next_layer_last;
                        seq[i - 1].push(val);
                    }
                }
            }

            if let Some(first) = seq.first() {
                if let Some(last) = first.last() {
                    self.ans = Some(*last)
                }
            }
        }
    }
}

fn main() {
    let time = Instant::now();
    let data = data::DATA;
    let test = data::test;
    //
    let mut oasis = oasis_data::new();
    oasis.fill_data(data, true); // change to false for part1
    let answer = oasis.part();
    //println!("{:?}", oasis);
    println!("{:?}", answer); // (answer part1 1479011877 // 803.7µs) (answer part2 973 1.0281ms)

    println!("{:?}", time.elapsed());
}
