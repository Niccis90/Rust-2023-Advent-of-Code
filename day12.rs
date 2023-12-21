use std::time::Instant;

mod data;

#[derive(Debug, Clone)]
struct spring_row {
    constraints: Vec<u8>,
    row: Vec<variable>,
}

#[derive(Debug, Clone)]
struct variable {
    constant: bool,
    works: Option<bool>,
}

impl variable {
    fn new(char: char) -> variable {
        let (constant, works) = match char {
            '#' => (true, Some(false)),
            '.' => (true, Some(true)),
            '?' => (false, None),
            _ => {
                eprintln!("Invalid character in data");
                (false, None)
            }
        };
        variable { constant, works }
    }
}

#[derive(Debug, Clone)]
struct springs {
    rows: Vec<spring_row>,
}

impl springs {
    fn new() -> springs {
        springs { rows: Vec::new() }
    }
    fn parse(&mut self, data: &str) {
        for line in data.lines() {
            let mut splitted = line.split_whitespace();
            let targets = splitted.next().unwrap();
            let numbers = splitted.next().unwrap();

            let constraints: Vec<u8> = numbers
                .split(',')
                .filter_map(|x| x.parse::<u8>().ok())
                .collect();
            let segments: Vec<variable> = targets.chars().map(|x| variable::new(x)).collect();

            let mut extended_constraints = Vec::new();
            let mut extended_segments = Vec::new();

            for _ in 0..5 {
                extended_constraints.extend(&constraints);
                extended_segments.extend(segments.clone());
            }

            self.rows.push(spring_row {
                constraints: extended_constraints,
                row: extended_segments,
            });
        }
    }
    fn generate_and_check(
        &self,
        variables: &mut [variable],
        constraints: &[u8],
        index: usize,
        count: &mut u128,
    ) {
        if index >= variables.len() {
            if self.is_valid_combination(variables, constraints) {
                *count += 1;
            }
            return;
        }

        if variables[index].constant {
            self.generate_and_check(variables, constraints, index + 1, count);
        } else {
            variables[index].works = Some(true);
            self.generate_and_check(variables, constraints, index + 1, count);

            variables[index].works = Some(false);
            self.generate_and_check(variables, constraints, index + 1, count);

            variables[index].works = None;
        }
    }
    fn is_valid_combination(&self, variables: &[variable], constraints: &[u8]) -> bool {
        let mut constraint_index = 0;
        let mut group_count = 0;

        for (i, var) in variables.iter().enumerate() {
            match var.works {
                Some(false) => {
                    group_count += 1;
                }
                _ => {
                    if group_count > 0 {
                        if constraint_index >= constraints.len()
                            || group_count != constraints[constraint_index]
                        {
                            return false;
                        }
                        constraint_index += 1;
                        group_count = 0;
                    }
                }
            }

            if group_count == 0
                && var.works == Some(true)
                && constraint_index < constraints.len()
                && i > 0
                && variables[i - 1].works == Some(false)
            {
                continue;
            } else if group_count == 0
                && var.works == Some(false)
                && constraint_index < constraints.len()
                && i > 0
                && variables[i - 1].works == Some(true)
            {
                continue;
            } else if group_count == 0 && var.works != Some(true) {
                return false;
            }
        }

        if group_count > 0 {
            if constraint_index >= constraints.len() || group_count != constraints[constraint_index]
            {
                return false;
            }
            constraint_index += 1;
        }

        constraint_index == constraints.len()
    }

    fn check_combinations(&self, constraints: Vec<u8>, variables: Vec<variable>) -> u128 {
        let mut num = 0;
        let mut temp_variables = variables.clone();
        self.generate_and_check(&mut temp_variables, &constraints, 0, &mut num);

        num
    }

    fn part1(&self) -> u128 {
        let mut num = 0;
        for row in &self.rows {
            println!("{:?}", num);
            num += self.check_combinations(row.constraints.clone(), row.row.clone());
        }
        num
    }
}

fn main() {
    let time = Instant::now();
    let test1 = data::TEST1;
    let data = data::DATA;
    let mut springs = springs::new();
    springs.parse(data);
    let part1 = springs.part1();
    println!("{:?}", part1);

    println!("{:?}", time.elapsed());
}
