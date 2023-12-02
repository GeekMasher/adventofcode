use crate::Arguments;
use anyhow::Result;

#[derive(Debug, Default, Clone)]
struct Game {
    id: i32,
    possible: bool,

    red: i32,
    blue: i32,
    green: i32,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut game = Game {
            possible: true,
            ..Default::default()
        };

        let (game_str, cubes) = value.split_once(": ").unwrap();
        game.id = game_str.replace("Game ", "").parse().unwrap();

        // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        cubes.split("; ").for_each(|subset| {
            subset.split(", ").for_each(|cube| {
                let (count_str, colour) = cube.split_once(' ').unwrap();
                let count: i32 = count_str.parse().unwrap();

                let result = match colour {
                    "red" => {
                        if game.red < count {
                            game.red = count
                        }
                        count > 12
                    }
                    "green" => {
                        if game.green < count {
                            game.green = count
                        }
                        count > 13
                    }
                    "blue" => {
                        if game.blue < count {
                            game.blue = count
                        }
                        count > 14
                    }
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

        let power = game.red * game.blue * game.green;

        println!("{:?} ({power})", game);

        // PART 2
        // if game.possible {
        //     total += game.id;
        // }
        total += power;
    }

    println!("Total :: {total}");

    Ok(())
}
