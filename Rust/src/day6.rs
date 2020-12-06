#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|group| parse_group(group))
        .collect()
}

const CHARCODE_SMALL_A: u32 = 97;

// Treat each character in a line as a bit flag
fn parse_group(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| 1 << ((c as u32) - CHARCODE_SMALL_A))
                .sum()
        })
        .collect()
}

fn count_set_bits(group: &Vec<u32>) -> u32 {
    group.iter().fold(0, |acc, v| acc | v).count_ones()
}

fn count_common_bits(group: &Vec<u32>) -> u32 {
    group.iter().fold(group[0], |acc, v| acc & v).count_ones()
}

#[aoc(day6, part1)]
pub fn part_one(groups: &Vec<Vec<u32>>) -> u32 {
    groups.iter().map(|g| count_set_bits(g)).sum()
}

#[aoc(day6, part2)]
pub fn count_common(groups: &Vec<Vec<u32>>) -> u32 {
    groups.iter().map(|g| count_common_bits(g)).sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_parses_groups() {
        let expected: Vec<u32> = vec![
            8388615,  // 1 | 2 | 4 | 8388608
            16777223, // 1 | 2 | 4 | 16777216
            33554439, // 1 | 2 | 4 | 33554432
        ];
        let input = "abcx\nabcy\nabcz";

        assert_eq!(parse_group(input), expected)
    }

    #[test]
    fn it_parses_multiple_groups() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

        assert_eq!(parse_input(input).len(), 5);
    }

    #[test]
    fn it_counts_bits_in_group() {
        let input: Vec<u32> = vec![8388615, 16777223, 33554439];

        assert_eq!(count_set_bits(&input), 6);
    }

    #[test]
    fn it_counts_common_bits_in_group() {
        let input: Vec<u32> = vec![8388615, 16777223, 33554439];

        assert_eq!(count_common_bits(&input), 3);
    }
}
