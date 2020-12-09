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
}
