use std::{f32::DIGITS, time::Instant};

mod data;

fn is_adjacent(groups: Vec<Vec<[i16; 2]>>, data_array: [[char; 140]; 140]) -> Vec<Vec<[i16; 2]>> {
    let mut adjacent_numbers = Vec::new();
    for group in groups.iter() {
        let mut group_added = false;
        for coord in group.iter() {
            if group_added {
                break;
            }
            let up_idx = [coord[0], coord[1] + 1];
            let down_idx = [coord[0], coord[1] - 1];
            let left_idx = [coord[0] - 1, coord[1]];
            let right_idx = [coord[0] + 1, coord[1]];
            let diagonal_ru = [coord[0] + 1, coord[1] + 1];
            let diagonal_rd = [coord[0] + 1, coord[1] - 1];
            let diagonal_lu = [coord[0] - 1, coord[1] + 1];
            let diagonal_ld = [coord[0] - 1, coord[1] - 1];
            let check1 = data_array
                .get(up_idx[1] as usize)
                .and_then(|r| r.get(up_idx[0] as usize));
            let check2 = data_array
                .get(down_idx[1] as usize)
                .and_then(|r| r.get(down_idx[0] as usize));
            let check3 = data_array
                .get(left_idx[1] as usize)
                .and_then(|r| r.get(left_idx[0] as usize));
            let check4 = data_array
                .get(right_idx[1] as usize)
                .and_then(|r| r.get(right_idx[0] as usize));
            let check5 = data_array
                .get(diagonal_ru[1] as usize)
                .and_then(|r| r.get(diagonal_ru[0] as usize));
            let check6 = data_array
                .get(diagonal_rd[1] as usize)
                .and_then(|r| r.get(diagonal_rd[0] as usize));
            let check7 = data_array
                .get(diagonal_lu[1] as usize)
                .and_then(|r| r.get(diagonal_lu[0] as usize));
            let check8 = data_array
                .get(diagonal_ld[1] as usize)
                .and_then(|r| r.get(diagonal_ld[0] as usize));
            let checklist = [
                check1, check2, check3, check4, check5, check6, check7, check8,
            ];
            for i in checklist.iter() {
                if let Some(c) = i {
                    if !c.is_digit(10) && *c != &'.' {
                        adjacent_numbers.push(group.clone());
                        group_added = true;
                        break;
                    }
                }
            }
        }
    }

    adjacent_numbers
}

fn part1(data: &str) -> u32 {
    let mut data_array: [[char; 140]; 140] = [[' '; 140]; 140];
    let mut coordinates: Vec<[i16; 2]> = Vec::new(); // [x,y] = [char_idx,line_index]
    for (line_index, line) in data.lines().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            data_array[line_index][char_idx] = char;
            if char.is_digit(10) {
                coordinates.push([char_idx as i16, line_index as i16])
            }
        }
    }
    let mut groups: Vec<Vec<[i16; 2]>> = Vec::new();
    let mut current_group: Vec<[i16; 2]> = Vec::new();
    for coord in coordinates.iter() {
        if current_group.is_empty()
            || (current_group.last().unwrap()[1] == coord[1]
                && (current_group.last().unwrap()[0] as i16 - coord[0] as i16).abs() == 1)
        {
            current_group.push(*coord);
        } else {
            if current_group.len() >= 1 {
                groups.push(current_group.clone());
            }
            current_group.clear();
            current_group.push(*coord);
        }
    }
    if current_group.len() >= 1 {
        groups.push(current_group);
    }
    //println!("{:?}",groups);
    let adjacent = is_adjacent(groups, data_array);
    //println!("{:?}", adjacent);
    let mut final_nums: Vec<u32> = Vec::new();
    for set in adjacent.iter() {
        if set.len() == 3 {
            let num1 = data_array[set[0][1] as usize][set[0][0] as usize];
            let num2 = data_array[set[1][1] as usize][set[1][0] as usize];
            let num3 = data_array[set[2][1] as usize][set[2][0] as usize];
            let temp = format!("{}{}{}", num1, num2, num3);
            final_nums.push(temp.parse().unwrap());
        }
        if set.len() == 2 {
            let num4 = data_array[set[0][1] as usize][set[0][0] as usize];
            let num5 = data_array[set[1][1] as usize][set[1][0] as usize];
            let temp2 = format!("{}{}", num4, num5);
            final_nums.push(temp2.parse().unwrap());
        }
        if set.len() == 1 {
            let num6 = data_array[set[0][1] as usize][set[0][0] as usize];
            //println!("{:?}",num6);
            let temp2 = format!("{}", num6);
            final_nums.push(temp2.parse().unwrap());
        }
    }
    //println!("{:?}",final_nums);
    let sum = final_nums.iter().sum();
    sum
}

fn is_gear(star_coordinates: Vec<[i16; 2]>, groups: Vec<Vec<[i16; 2]>>) -> Vec<Vec<Vec<[i16; 2]>>> {
    let mut gears: Vec<Vec<Vec<[i16; 2]>>> = Vec::new();
    for star in star_coordinates.iter() {
        let mut temp: Vec<Vec<[i16; 2]>> = Vec::new();
        let mut added_group_indices: Vec<usize> = Vec::new();

        for (index, group) in groups.iter().enumerate() {
            for coord in group.iter() {
                let adjacent_positions = [
                    [star[0], star[1] + 1],
                    [star[0], star[1] - 1],
                    [star[0] - 1, star[1]],
                    [star[0] + 1, star[1]],
                    [star[0] + 1, star[1] + 1],
                    [star[0] + 1, star[1] - 1],
                    [star[0] - 1, star[1] + 1],
                    [star[0] - 1, star[1] - 1],
                ];

                if adjacent_positions.contains(coord) && !added_group_indices.contains(&index) {
                    temp.push(group.clone());
                    added_group_indices.push(index);
                }
            }
        }

        if temp.len() == 2 {
            gears.push(temp);
        }
    }
    gears
}

fn to_numbers(gears: Vec<Vec<Vec<[i16; 2]>>>, data_array: [[char; 140]; 140]) -> Vec<Vec<u32>> {
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    for gear in gears.iter() {
        let mut parts: Vec<u32> = Vec::new();
        for set in gear.iter() {
            if set.len() == 3 {
                let num1 = data_array[set[0][1] as usize][set[0][0] as usize];
                let num2 = data_array[set[1][1] as usize][set[1][0] as usize];
                let num3 = data_array[set[2][1] as usize][set[2][0] as usize];
                let temp = format!("{}{}{}", num1, num2, num3);
                parts.push(temp.parse().unwrap());
            }
            if set.len() == 2 {
                let num4 = data_array[set[0][1] as usize][set[0][0] as usize];
                let num5 = data_array[set[1][1] as usize][set[1][0] as usize];
                let temp2 = format!("{}{}", num4, num5);
                parts.push(temp2.parse().unwrap());
            }
            if set.len() == 1 {
                let num6 = data_array[set[0][1] as usize][set[0][0] as usize];
                //println!("{:?}",num6);
                let temp2 = format!("{}", num6);
                parts.push(temp2.parse().unwrap());
            }
        }
        numbers.push(parts);
    }

    numbers
}

fn part2(data: &str) -> u32 {
    let mut data_array: [[char; 140]; 140] = [[' '; 140]; 140];
    let mut coordinates: Vec<[i16; 2]> = Vec::new(); // [x,y] = [char_idx,line_index]
    let mut star_coordinates: Vec<[i16; 2]> = Vec::new();
    for (line_index, line) in data.lines().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            data_array[line_index][char_idx] = char;
            if char.is_digit(10) {
                coordinates.push([char_idx as i16, line_index as i16])
            }
            if char == '*' {
                star_coordinates.push([char_idx as i16, line_index as i16]);
            }
        }
    }
    let mut groups: Vec<Vec<[i16; 2]>> = Vec::new();
    let mut current_group: Vec<[i16; 2]> = Vec::new();
    for coord in coordinates.iter() {
        if current_group.is_empty()
            || (current_group.last().unwrap()[1] == coord[1]
                && (current_group.last().unwrap()[0] as i16 - coord[0] as i16).abs() == 1)
        {
            current_group.push(*coord);
        } else {
            if current_group.len() >= 1 {
                groups.push(current_group.clone());
            }
            current_group.clear();
            current_group.push(*coord);
        }
    }
    if current_group.len() >= 1 {
        groups.push(current_group);
    }
    //println!("group: {:?}", groups);
    //println!("star_coords: {:?}",star_coordinates);
    let gears: Vec<Vec<Vec<[i16; 2]>>> = is_gear(star_coordinates, groups);
    //println!("{:?}", gears);
    let numbers = to_numbers(gears, data_array);
    //println!("{:?}", numbers);
    let mut sum = 0;
    for i in numbers.iter() {
        sum += i[0] * i[1]
    }

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
