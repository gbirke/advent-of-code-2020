#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|n| n.parse::<i32>().unwrap()).collect()
}

fn difference_sums(numbers: &[i32]) -> (i32, i32) {
    let mut sorted_numbers: Vec<i32> = Vec::new();
    sorted_numbers.extend_from_slice(numbers);
    sorted_numbers.sort();
    let mut sorted_numbers_iter = sorted_numbers.iter();
    let start_number = sorted_numbers_iter.next().unwrap();
    let result = sorted_numbers.iter().fold(
        (start_number, 1, 1),
        |(prev_number, one_jolt, three_jolts), n| match n - prev_number {
            1 => (n, one_jolt + 1, three_jolts),
            3 => (n, one_jolt, three_jolts + 1),
            _ => (n, one_jolt, three_jolts),
        },
    );
    (result.1, result.2)
}

#[aoc(day10, part2)]
pub fn part_one(numbers: &[i32]) -> i32 {
    let result = difference_sums(numbers);
    result.0 * result.1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_collects_differences() {
        let input1: Vec<i32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let input2: Vec<i32> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(difference_sums(&input1), (7, 5));
        assert_eq!(difference_sums(&input2), (22, 10))
    }
}
