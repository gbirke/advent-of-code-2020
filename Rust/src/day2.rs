#[derive(Eq, PartialEq, Debug)]
pub struct PasswordAndPolicy {
    c: char,
    lower_bound: i32,
    upper_bound: i32,
    password: String,
}

pub fn parse_input(input: &str) -> Vec<PasswordAndPolicy> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let mut upper_and_lower = parts.next().unwrap().split("-");
            let lower_bound: i32 = upper_and_lower.next().unwrap().parse::<i32>().unwrap();
            let upper_bound: i32 = upper_and_lower.next().unwrap().parse::<i32>().unwrap();
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
}
