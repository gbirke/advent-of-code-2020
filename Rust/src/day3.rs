#[derive(Eq, PartialEq, Debug)]
pub struct TreePattern {
    pattern: Vec<bool>,
    width: usize,
    height: usize,
}

impl TreePattern {
    fn has_tree(&self, x: usize, y: usize) -> bool {
        return self.pattern[y * self.width + x % self.width];
    }

    fn count_trees(&self, x_step: usize, y_step: usize) -> usize {
        let mut x: usize = 0;
        let mut tree_count: usize = 0;
        for y in (0..self.height).step_by(y_step) {
            if self.has_tree(x, y) {
                tree_count += 1
            }
            x += x_step;
        }
        tree_count
    }
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> TreePattern {
    let mut height: usize = 0;
    let mut pattern: Vec<bool> = Vec::new();
    for line in input.lines() {
        pattern.append(&mut line.chars().map(|c| c == '#').collect::<Vec<bool>>());
        height += 1;
    }
    let width = input.lines().next().unwrap().len();
    TreePattern {
        pattern,
        width,
        height,
    }
}

#[aoc(day3, part1)]
pub fn solve_part1(trees: &TreePattern) -> usize {
    trees.count_trees(3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(trees: &TreePattern) -> usize {
    let slope_params = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slope_params
        .iter()
        .map(|p| trees.count_trees(p.0, p.1))
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PATTERN: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn it_parses() {
        let expected = TreePattern {
            pattern: vec![
                false, false, true, true, false, false, false, false, false, false, false, true,
                false, false, false, true, false, false, false, true, false, false, false, true,
                false, false, false, false, true, false, false, true, false, false, false, true,
                false, true, false, false, false, true, false, true, false, true, false, false,
                false, true, true, false, false, true, false, false, false, true, false, true,
                true, false, false, false, false, false, false, true, false, true, false, true,
                false, false, false, false, true, false, true, false, false, false, false, false,
                false, false, false, true, true, false, true, true, false, false, false, true,
                false, false, false, true, false, false, false, true, true, false, false, false,
                false, true, false, true, false, false, true, false, false, false, true, false,
                true,
            ],
            width: 11,
            height: 11,
        };
        assert_eq!(expected, parse_input(TEST_PATTERN))
    }

    #[test]
    fn it_accesses_coordinates() {
        let trees = TreePattern {
            pattern: vec![
                false, false, true, true, false, false, false, false, false, false, false, true,
                false, false, false, true, false, false, false, true, false, false, false, true,
                false, false, false, false, true, false, false, true, false, false, false, true,
                false, true, false, false, false, true, false, true, false, true, false, false,
                false, true, true, false, false, true, false, false, false, true, false, true,
                true, false, false, false, false, false, false, true, false, true, false, true,
                false, false, false, false, true, false, true, false, false, false, false, false,
                false, false, false, true, true, false, true, true, false, false, false, true,
                false, false, false, true, false, false, false, true, true, false, false, false,
                false, true, false, true, false, false, true, false, false, false, true, false,
                true,
            ],
            width: 11,
            height: 11,
        };
        assert!(!trees.has_tree(0, 0));
        assert!(trees.has_tree(2, 0));
        assert!(trees.has_tree(0, 1));
        assert!(trees.has_tree(10, 10));
    }

    #[test]
    fn it_counts_trees() {
        let trees = TreePattern {
            pattern: vec![
                false, false, true, true, false, false, false, false, false, false, false, true,
                false, false, false, true, false, false, false, true, false, false, false, true,
                false, false, false, false, true, false, false, true, false, false, false, true,
                false, true, false, false, false, true, false, true, false, true, false, false,
                false, true, true, false, false, true, false, false, false, true, false, true,
                true, false, false, false, false, false, false, true, false, true, false, true,
                false, false, false, false, true, false, true, false, false, false, false, false,
                false, false, false, true, true, false, true, true, false, false, false, true,
                false, false, false, true, false, false, false, true, true, false, false, false,
                false, true, false, true, false, false, true, false, false, false, true, false,
                true,
            ],
            width: 11,
            height: 11,
        };
        assert_eq!(7, trees.count_trees(3, 1));
    }

    #[test]
    fn it_tries_slopes() {
        let trees = TreePattern {
            pattern: vec![
                false, false, true, true, false, false, false, false, false, false, false, true,
                false, false, false, true, false, false, false, true, false, false, false, true,
                false, false, false, false, true, false, false, true, false, false, false, true,
                false, true, false, false, false, true, false, true, false, true, false, false,
                false, true, true, false, false, true, false, false, false, true, false, true,
                true, false, false, false, false, false, false, true, false, true, false, true,
                false, false, false, false, true, false, true, false, false, false, false, false,
                false, false, false, true, true, false, true, true, false, false, false, true,
                false, false, false, true, false, false, false, true, true, false, false, false,
                false, true, false, true, false, false, true, false, false, false, true, false,
                true,
            ],
            width: 11,
            height: 11,
        };
        assert_eq!(336, solve_part2(&trees));
    }
}
