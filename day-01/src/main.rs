use std::collections::HashMap;
use std::fs;
use std::iter;

fn read_input_numbers() -> Vec<i64> {
    let content = fs::read_to_string("input.txt").expect("input.txt not found");
    content.lines().map(|l| l.parse::<i64>().unwrap()).collect()
}

fn part1(numbers: &[i64]) {
    // a lookup table for [2020-n] => n
    let numbers_and_rests: HashMap<_, _> = numbers
        .iter()
        .map(|n| 2020 - n)
        .zip(numbers.iter().map(|n| *n))
        .collect();

    let solution = numbers_and_rests
        .iter()
        .find(|(_counterpart, number)| numbers_and_rests.contains_key(number))
        .unwrap();

    println!(
        "Part 1: found {} and {}, product is {}",
        solution.0,
        solution.1,
        solution.0 * *solution.1
    );
}

fn part2(numbers: &[i64]) {
    let lowest_number = numbers.iter().min().unwrap();
    // Create tuples of the cartesian product of numbers, leaving out existing
    // combinations by using an index.
    // Then create a combined tuple of the first tuple (2020-n1-n2, n1, n2 )
    // Filter out all items of the first tuple that are smaller than the lowest number
    let mut two_number_combinations = numbers.iter().enumerate().flat_map(|(i, n1)| {
        numbers[i + 1..]
            .iter()
            .map(move |n2| (2020 - n1 - n2, *n1, *n2))
            .filter(|p| p.0 >= *lowest_number)
    });
    // A lookup table for the missing 3rd number
    let num_lookup: HashMap<_, _> = numbers.iter().zip(iter::repeat(1)).collect();
    let solution = two_number_combinations
        .find(|(rest, _n1, _n2)| num_lookup.contains_key(rest))
        .unwrap();
    println!(
        "Part 2: found {}, {} and {}, product is {}",
        solution.0,
        solution.1,
        solution.2,
        solution.0 * solution.1 * solution.2
    );
}

fn main() {
    // let test_numbers = vec![1721, 979, 366, 299, 675, 145];
    let input_numbers = &read_input_numbers();
    part1(input_numbers);
    part2(input_numbers);
}
