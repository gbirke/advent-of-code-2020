use std::collections::HashMap;
use std::fs;

fn read_input_numbers() -> Vec<i64> {
    let content = fs::read_to_string("input.txt").expect("input.txt not found");
    content.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

fn part1(numbers: &[i64]) {
    let number_counterparts: HashMap<_, _> = numbers
        .iter()
        .map(|n| 2020 - n)
        .zip(numbers.iter())
        .collect();
    let solution = number_counterparts
        .iter()
        .find(|(_counterpart, number)| number_counterparts.contains_key(number))
        .unwrap();
    println!(
        "found {} and {}, product is {}",
        solution.0,
        solution.1,
        solution.0 * *solution.1
    );
}

fn main() {
    part1(&read_input_numbers())
}
