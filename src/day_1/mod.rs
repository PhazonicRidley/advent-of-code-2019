use aoc_setup;

pub fn solve() {
    println!("Hi from  day 1");
    let puzzle_input = aoc_setup::get_puzzle_data(2019, 1, "\n");
    println!("Part 1: {}", part_1(&puzzle_input));
    println!("Part 2: {}", part_2(&puzzle_input));
}

fn part_1(puzzle_input: &Vec<String>) -> f32 {
    return puzzle_input
        .iter()
        .map(|s| (s.parse::<f32>().unwrap() / 3.0).floor() - 2.0)
        .sum();
}

fn part_2(puzzle_input: &Vec<String>) -> i32 {
    let mut total_mass = 0;
    for i in puzzle_input.iter().map(|s| s.parse::<i32>().unwrap()) {
        total_mass += calculate_fuel(i);
    }

    return total_mass;
}

fn calculate_fuel(module: i32) -> i32 {
    let mut remaining_mass = module;
    let mut fuel_mass = 0;
    loop {
        let fuel = (remaining_mass / 3) - 2;
        if fuel < 0 {
            break;
        }
        fuel_mass += fuel;
        remaining_mass = fuel;
    }

    return fuel_mass;
}
