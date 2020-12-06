use std::collections::HashMap;
use std::convert::TryFrom;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<HashMap<char, u32>> {
    input
        .split("\n\n")
        .map(|group| parse_group(group))
        .collect()
}

fn parse_group(input: &str) -> HashMap<char, u32> {
    let mut result: HashMap<char, u32> = HashMap::new();
    for line in input.lines() {
        for c in line.chars() {
            let new_count = match result.get(&c) {
                Some(count) => *count + 1,
                None => 1,
            };
            result.insert(c, new_count);
        }
    }
    // Store number of group members (i.e. lines)
    result.insert('#', u32::try_from(input.lines().count()).unwrap());
    result
}

#[aoc(day6, part1)]
pub fn part_one(groups: &Vec<HashMap<char, u32>>) -> usize {
    groups.iter().map(|g| g.len()).sum()
}

#[aoc(day6, part2)]
pub fn count_common(groups: &Vec<HashMap<char, u32>>) -> usize {
    groups
        .iter()
        .map(|g| {
            g.iter()
                .filter(|(&c, &v)| c != '#' && v == *g.get(&'#').unwrap())
                .count()
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_parses_groups() {
        let expected: HashMap<char, u32> = [
            ('a', 3),
            ('b', 3),
            ('c', 3),
            ('x', 1),
            ('y', 1),
            ('z', 1),
            ('#', 3),
        ]
        .iter()
        .cloned()
        .collect();
        let input = "abcx\nabcy\nabcz";

        assert_eq!(parse_group(input), expected)
    }

    #[test]
    fn it_parses_multiple_groups() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

        assert_eq!(parse_input(input).len(), 4);
        assert_eq!(
            parse_input(input).iter().map(|g| g.len()).sum::<usize>(),
            11
        );
    }

    #[test]
    fn it_counts_common() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

        assert_eq!(count_common(&parse_input(input)), 6)
    }
}
