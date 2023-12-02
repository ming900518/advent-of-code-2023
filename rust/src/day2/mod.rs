use self::input::INPUT;

mod input;

pub fn start() {
    println!("Day 2 Part 1: {}", part1(INPUT));
    println!("Day 2 Part 2: {}", part2(INPUT));
}

enum ColorCubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl TryFrom<&str> for ColorCubes {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        if value.ends_with(" red") {
            Ok(Self::Red(
                value
                    .strip_suffix(" red")
                    .and_then(|v| str::parse(v).ok())
                    .unwrap_or_default(),
            ))
        } else if value.ends_with(" green") {
            Ok(Self::Green(
                value
                    .strip_suffix(" green")
                    .and_then(|v| str::parse(v).ok())
                    .unwrap_or_default(),
            ))
        } else if value.ends_with(" blue") {
            Ok(Self::Blue(
                value
                    .strip_suffix(" blue")
                    .and_then(|v| str::parse(v).ok())
                    .unwrap_or_default(),
            ))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

impl From<Game> for u32 {
    fn from(val: Game) -> Self {
        val.id
    }
}

impl TryFrom<&str> for Game {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        let Some((id, sets)) = value.split_once("Game ").and_then(|(_, content)| {
            content
                .split_once(": ")
                .and_then(|(id_str, content)| id_str.parse().map(|id| (id, content)).ok())
        }) else {
            return Err(());
        };

        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        sets.split("; ").for_each(|cubes| {
            cubes
                .split(", ")
                .filter_map(|raw_cube| ColorCubes::try_from(raw_cube).ok())
                .for_each(|processed_cube| match processed_cube {
                    ColorCubes::Red(count) if count > red_count => red_count = count,
                    ColorCubes::Green(count) if count > green_count => green_count = count,
                    ColorCubes::Blue(count) if count > blue_count => blue_count = count,
                    _ => (),
                });
        });

        Ok(Self {
            id,
            red_count,
            green_count,
            blue_count,
        })
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            Game::try_from(line).ok().and_then(
                |Game {
                     id,
                     red_count,
                     green_count,
                     blue_count,
                 }| {
                    (red_count <= 12 && green_count <= 13 && blue_count <= 14).then_some(id)
                },
            )
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let Ok(Game {
                id: _,
                red_count,
                green_count,
                blue_count,
            }) = Game::try_from(line)
            else {
                return 0;
            };
            red_count * green_count * blue_count
        })
        .sum()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1() {
        assert_eq!(crate::day2::part1(TEST_INPUT), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::day2::part2(TEST_INPUT), 2286);
    }
}
