#[derive(Eq, PartialEq, Debug)]
pub struct BusSchedule {
    timestamp: i32,
    buses: Vec<Option<i32>>,
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> BusSchedule {
    let mut lines = input.lines();
    let timestamp: i32 = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(",")
        .map(|b| b.parse::<i32>().ok())
        .collect();
    BusSchedule { timestamp, buses }
}

fn get_next_departures(schedule: &BusSchedule) -> Vec<(i32, i32)> {
    schedule
        .buses
        .iter()
        .filter_map(|b| match b {
            Some(bus_id) => Some((
                *bus_id,
                bus_id + schedule.timestamp - (schedule.timestamp % bus_id),
            )),
            None => None,
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part_one(schedule: &BusSchedule) -> i32 {
    let next_departures = get_next_departures(schedule);
    let fastest = next_departures
        .iter()
        .min_by(|b1, b2| b1.1.cmp(&b2.1))
        .unwrap();
    let minutes_to_wait = fastest.1 - schedule.timestamp;
    fastest.0 * minutes_to_wait
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_schedule() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let expected = BusSchedule {
            timestamp: 939,
            buses: vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19),
            ],
        };

        assert_eq!(parse_input(input), expected)
    }

    #[test]
    fn it_gets_next_departures() {
        let schedule = BusSchedule {
            timestamp: 939,
            buses: vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19),
            ],
        };
        let expected = vec![(7, 945), (13, 949), (59, 944), (31, 961), (19, 950)];

        assert_eq!(get_next_departures(&schedule), expected)
    }

    #[test]
    fn it_solves_day_13_part_one_with_test_data() {
        let schedule = BusSchedule {
            timestamp: 939,
            buses: vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19),
            ],
        };

        assert_eq!(part_one(&schedule), 295)
    }
}
