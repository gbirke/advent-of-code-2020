#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| {
            let binary_string = l
                .chars()
                .map(|c| match c {
                    'F' => '0',
                    'B' => '1',
                    'L' => '0',
                    'R' => '1',
                    _ => c,
                })
                .collect::<String>();
            u32::from_str_radix(&binary_string, 2).unwrap()
        })
        .collect::<Vec<u32>>()
}

#[aoc(day5, part1)]
pub fn max_seat_id(seat_ids: &Vec<u32>) -> u32 {
    *seat_ids.iter().max().unwrap()
}

// The get_col and get_row functions don't serve a purpose
// I used them to understand the seat numbering system

#[allow(dead_code)]
fn get_col(seat_id: u32) -> u32 {
    seat_id & 0b0000000111
}

#[allow(dead_code)]
fn get_row(seat_id: u32) -> u32 {
    seat_id >> 3
}

#[aoc(day5, part2)]
pub fn my_seat_id(seat_ids: &Vec<u32>) -> Option<u32> {
    let mut ordered_seat_ids = seat_ids.clone();
    ordered_seat_ids.sort();
    for i in 1..ordered_seat_ids.len() {
        // When there is a gap of more than 1 in the seat numbers,
        // we found the right one
        let seat_number_difference = ordered_seat_ids[i] - ordered_seat_ids[i - 1];
        if seat_number_difference > 1 {
            return Some(ordered_seat_ids[i] - 1);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_converts_inputs() {
        let expected: Vec<u32> = vec![567, 119, 820];
        let input = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";

        assert_eq!(expected, parse_input(input));
    }

    #[test]
    fn it_gets_cols() {
        assert_eq!(7, get_col(567));
        assert_eq!(7, get_col(119));
        assert_eq!(4, get_col(820));
    }

    #[test]
    fn it_gets_rows() {
        assert_eq!(70, get_row(567));
        assert_eq!(14, get_row(119));
        assert_eq!(102, get_row(820));
    }
}
