use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|n| n.parse::<i64>().unwrap()).collect()
}

fn is_valid(number: i64, search_list: &[i64]) -> bool {
    let lookup_list: HashSet<&i64> = search_list.iter().collect();
    search_list
        .iter()
        .find(|i| {
            let complement = number - *i;
            match lookup_list.get(&complement) {
                Some(found_complement) => found_complement != i,
                None => false,
            }
        })
        .is_some()
}

fn detect_invalid_in_range(numbers: &[i64], window_size: usize) -> Option<i64> {
    let index_number: usize = window_size + 1;
    for i in index_number..numbers.len() {
        if !is_valid(numbers[i], &numbers[i - index_number..i]) {
            return Some(numbers[i]);
        }
    }
    None
}

#[aoc(day9, part1)]
pub fn part_one(numbers: &[i64]) -> Option<i64> {
    detect_invalid_in_range(numbers, 25)
}

fn find_numbers_in_range(
    numbers: &[i64],
    search_number: i64,
    initial_window_size: usize,
) -> Option<Vec<&i64>> {
    let mut window_size = initial_window_size;
    while window_size > 1 {
        let mut i = numbers.len();
        let optimization_threshold = search_number / window_size as i64;
        while i > window_size {
            let sum: i64 = numbers[i - window_size..i].iter().sum();
            if sum == search_number {
                return Some(numbers[i - window_size..i].iter().collect());
            }
            // possible optimization because our numbers are ascending - stop searching early
            if sum < optimization_threshold {
                break;
            }
            i -= 1;
        }
        window_size -= 1;
    }
    None
}

#[aoc(day9, part2)]
pub fn part_two(numbers: &[i64]) -> Option<i64> {
    let invalid_number = detect_invalid_in_range(numbers, 25).unwrap();
    let start_position = numbers.iter().position(|n| n == &invalid_number).unwrap();
    match find_numbers_in_range(&numbers[..start_position], invalid_number, 25) {
        Some(found_range) => {
            Some(*found_range.iter().max().unwrap() + *found_range.iter().min().unwrap())
        }
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_checks_for_valid_number() {
        let input_numbers: Vec<i64> = (1..26).collect();
        for (number, expected) in [(26, true), (49, true), (50, false), (100, false)].iter() {
            assert_eq!(&is_valid(*number, &input_numbers), expected)
        }
    }

    #[test]
    fn it_checks_range_for_valid_numbers() {
        let input_numbers: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(detect_invalid_in_range(&input_numbers, 5), Some(127));
    }

    #[test]
    fn it_finds_sum_in_number_range() {
        let input_numbers: Vec<i64> =
            vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182];

        assert_eq!(
            find_numbers_in_range(&input_numbers, 127, 8),
            Some(vec![&15, &25, &47, &40])
        );
    }
}
