use crate::Arguments;
use anyhow::Result;

fn number_split(data: &str) -> Vec<i32> {
    data.split(' ')
        .filter_map(|r| match r.parse::<i32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect()
}

#[derive(Debug, Default)]
struct Race {
    time: i32,
    distance: i32,
}

impl Race {
    pub fn wins(&self) -> Vec<i32> {
        let mut wins: Vec<i32> = Vec::new();

        for hold in 1..self.time {
            let distance = hold * (self.time - hold);

            if distance > self.distance {
                wins.push(hold);
            }
        }

        wins
    }
}

fn build(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();
    let lines: Vec<&str> = input.split('\n').collect();

    let times: Vec<i32> = number_split(&lines[0].replace("Time:", ""));
    let distances: Vec<i32> = number_split(&lines[1].replace("Distance:", ""));

    for (i, time) in times.iter().enumerate() {
        races.push(Race {
            time: *time,
            distance: distances[i],
        })
    }

    races
}

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;

    let races = build(&input);

    println!("{:#?}", races);

    let wins: usize = races.iter().map(|r| r.wins().len()).product();

    println!("{wins}");

    Ok(())
}
