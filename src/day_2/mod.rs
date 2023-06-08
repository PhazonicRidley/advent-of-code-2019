use aoc_setup;
use std::vec;

pub fn solve() {
    let puzzle_input: Vec<i32> = aoc_setup::get_puzzle_data(2019, 2, ",")
        .iter()
        .filter(|s| s.parse::<i32>().is_ok())
        .map(|i| i.parse().unwrap())
        .collect();

    //println!("{:?}", puzzle_input.len());
    part_1(&mut puzzle_input.clone());
}

fn part_1(program: &mut Vec<i32>) -> i32 {
    let mut counter_idx = 0;

    while counter_idx < program.len() {
        let line: Vec<usize> = program
            .iter()
            .skip(counter_idx)
            .take(4)
            .map(|i| i.clone() as usize)
            .collect();

        if line.len() < 4 {
            break;
        }

        let operator = line[0];
        let first_operand_idx = line[1];
        let second_operand_idx = line[2];
        let answer_idx = line[3];

        match operator {
            1 => program[answer_idx] = program[first_operand_idx] + program[second_operand_idx],
            2 => program[answer_idx] = program[first_operand_idx] * program[second_operand_idx],
            _ => {
                println!(
                    "Breaking out of program op code is: {}, position is: {}",
                    operator, counter_idx
                );
                break;
            }
        }

        counter_idx += 4;
    }

    println!("{:?}", program);
    return program[0];
}
