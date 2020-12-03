#[derive(Eq, PartialEq, Debug)]
pub struct PasswordAndPolicy {
    c: char,
    lower_bound: usize,
    upper_bound: usize,
    password: String,
}

impl PasswordAndPolicy {
    fn password_is_valid(&self) -> bool {
        let char_count = self.password.chars().filter(|c| *c == self.c).count();
        char_count >= self.lower_bound && char_count <= self.upper_bound
    }

    fn password_is_valid_for_new_policy(&self) -> bool {
        let first = nth_char(&self.password, self.lower_bound - 1);
        let second = nth_char(&self.password, self.upper_bound - 1);
        // find only one:
        // compare false == false if none match and true == true if both match
        (first == self.c) != (second == self.c)
    }
}

fn nth_char(password: &str, index: usize) -> char {
    password.chars().nth(index).unwrap_or_default()
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<PasswordAndPolicy> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let mut upper_and_lower = parts.next().unwrap().split("-");
            let lower_bound: usize = upper_and_lower.next().unwrap().parse::<usize>().unwrap();
            let upper_bound: usize = upper_and_lower.next().unwrap().parse::<usize>().unwrap();
            let c = parts.next().unwrap().chars().next().unwrap();
            let password = parts.next().unwrap();

            PasswordAndPolicy {
                c,
                lower_bound,
                upper_bound,
                password: password.to_string(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn count_valid_passwords(entries: &[PasswordAndPolicy]) -> usize {
    entries.iter().filter(|e| e.password_is_valid()).count()
}

#[aoc(day2, part2)]
pub fn count_valid_passwords_for_new_policy(entries: &[PasswordAndPolicy]) -> usize {
    entries
        .iter()
        .filter(|e| e.password_is_valid_for_new_policy())
        .count()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_one_input() {
        let input = "1-3 a: abcd";
        let expected = PasswordAndPolicy {
            c: 'a',
            lower_bound: 1,
            upper_bound: 3,
            password: "abcd".to_string(),
        };

        assert_eq!(vec![expected], parse_input(input));
    }

    #[test]
    fn it_parses_multiple_inputs() {
        let input = "1-3 a: abcd\n4-5 x: xxyy";
        let expected = vec![
            PasswordAndPolicy {
                c: 'a',
                lower_bound: 1,
                upper_bound: 3,
                password: "abcd".to_string(),
            },
            PasswordAndPolicy {
                c: 'x',
                lower_bound: 4,
                upper_bound: 5,
                password: "xxyy".to_string(),
            },
        ];

        assert_eq!(expected, parse_input(input));
    }

    #[test]
    fn it_checks_password_validity() {
        let should_be_valid = PasswordAndPolicy {
            c: 'a',
            lower_bound: 1,
            upper_bound: 3,
            password: "abcd".to_string(),
        };
        let should_be_invalid = PasswordAndPolicy {
            c: 'x',
            lower_bound: 4,
            upper_bound: 5,
            password: "xxyy".to_string(),
        };
        assert!(should_be_valid.password_is_valid());
        assert!(!should_be_invalid.password_is_valid());
    }

    #[test]
    fn it_returns_char_at() {
        assert_eq!('a', nth_char("abcd", 0));
        assert_eq!('b', nth_char("abcd", 1));
        assert_eq!('\0', nth_char("abcd", 5));
    }

    #[test]
    fn it_checks_password_validity_for_new_policy() {
        let should_be_valid = PasswordAndPolicy {
            c: 'a',
            lower_bound: 1,
            upper_bound: 3,
            password: "abcd".to_string(),
        };
        let should_be_invalid1 = PasswordAndPolicy {
            c: 'b',
            lower_bound: 1,
            upper_bound: 3,
            password: "cdefg".to_string(),
        };
        let should_be_invalid2 = PasswordAndPolicy {
            c: 'c',
            lower_bound: 2,
            upper_bound: 9,
            password: "ccccccccc".to_string(),
        };

        assert!(should_be_valid.password_is_valid_for_new_policy());
        assert!(!should_be_invalid1.password_is_valid_for_new_policy());
        assert!(!should_be_invalid2.password_is_valid_for_new_policy());
    }
}
