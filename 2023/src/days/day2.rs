use crate::Arguments;
use anyhow::Result;

#[derive(Debug, Default, Clone)]
struct Game {
    id: i32,
    possible: bool,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut game = Game {
            id: 0,
            possible: true,
        };

        let (game_str, cubes) = value.split_once(": ").unwrap();
        game.id = game_str.replace("Game ", "").parse().unwrap();

        // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        cubes.split("; ").for_each(|subset| {
            subset.split(", ").for_each(|cube| {
                let (count_str, colour) = cube.split_once(' ').unwrap();
                let count: i32 = count_str.parse().unwrap();

                let result = match colour {
                    "red" => count > 12,
                    "green" => count > 13,
                    "blue" => count > 14,
                    _ => todo!("More colours?"),
                };
                if result {
                    game.possible = false
                }
            });
        });

        game
    }
}

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;
    let mut total = 0;

    for line in input.lines() {
        let game = Game::from(line);

        if game.possible {
            total += game.id;
        }
    }

    println!("Total :: {total}");

    Ok(())
}
