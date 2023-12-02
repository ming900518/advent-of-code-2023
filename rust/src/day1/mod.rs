use self::input::INPUT;

pub mod input;

pub fn start() {
    println!("Day 1 Part 1: {}", part1(INPUT));
    println!("Day 1 Part 2: {}", part2(INPUT));
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter(|&v| char::is_numeric(v));
            match (iter.next(), iter.next_back()) {
                (Some(x), Some(y)) => format!("{x}{y}"),
                (Some(x), None) => format!("{x}{x}"),
                (None, Some(y)) => format!("{y}{y}"),
                (None, None) => String::from("0"),
            }
            .parse::<u32>()
            .unwrap_or_default()
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    #[repr(u32)]
    #[derive(Clone, Copy)]
    enum Digit {
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Six = 6,
        Seven = 7,
        Eight = 8,
        Nine = 9,
    }

    impl<'a> From<Digit> for &'a str {
        fn from(val: Digit) -> Self {
            match val {
                Digit::One => "one",
                Digit::Two => "two",
                Digit::Three => "three",
                Digit::Four => "four",
                Digit::Five => "five",
                Digit::Six => "six",
                Digit::Seven => "seven",
                Digit::Eight => "eight",
                Digit::Nine => "nine",
            }
        }
    }

    impl TryFrom<u32> for Digit {
        type Error = ();

        fn try_from(value: u32) -> Result<Self, ()> {
            match value {
                1 => Ok(Self::One),
                2 => Ok(Self::Two),
                3 => Ok(Self::Three),
                4 => Ok(Self::Four),
                5 => Ok(Self::Five),
                6 => Ok(Self::Six),
                7 => Ok(Self::Seven),
                8 => Ok(Self::Eight),
                9 => Ok(Self::Nine),
                _ => Err(()),
            }
        }
    }

    fn find(input: &str) -> u32 {
        let mut input = input;
        let mut start = None;
        let mut end = None;

        while !input.is_empty() {
            let mut remove_from_front = 1;
            let mut remove_from_back = 1;

            for i in 1_u32..10_u32 {
                let digit = Digit::try_from(i).unwrap();
                let digit_str = digit.into();
                if start.is_none() {
                    if input.starts_with::<&str>(digit_str) {
                        start = Some(digit as u32);
                        remove_from_front = digit_str.len();
                    } else if input.starts_with::<&str>(&i.to_string()) {
                        start = Some(digit as u32);
                    }
                }
                if end.is_none() {
                    if input.ends_with::<&str>(digit_str) {
                        end = Some(digit as u32);
                        remove_from_back = digit_str.len();
                    } else if input.ends_with::<&str>(&i.to_string()) {
                        end = Some(digit as u32);
                    }
                }
            }
            let mut chars = input.chars();
            if start.is_none() {
                for _ in 0..remove_from_front {
                    chars.next();
                }
            }
            if end.is_none() {
                for _ in 0..remove_from_back {
                    chars.next_back();
                }
            }
            input = chars.as_str();
            if start.is_some_and(|_| end.is_some()) {
                break;
            }
        }

        match (start, end) {
            (Some(x), Some(y)) => x * 10 + y,
            (Some(x), None) => x * 10 + x,
            (None, Some(y)) => y * 10 + y,
            (None, None) => 0,
        }
    }

    input.lines().map(find).sum()
}

#[cfg(test)]
mod test {
    use crate::day1::{input::TEST_INPUT_1, input::TEST_INPUT_2, part1, part2};

    #[test]
    fn part_1() {
        assert_eq!(part1(TEST_INPUT_1), 142);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(TEST_INPUT_2), 281);
    }
}
