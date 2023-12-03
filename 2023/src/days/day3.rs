use std::collections::HashSet;

use crate::Arguments;
use anyhow::Result;

type Point = (i32, i32, char);

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Number {
    value: String, // 467
    row: i32,
    columns: Vec<i32>,
}

#[derive(Debug, Default)]
struct Engine {
    pub numbers: Vec<Number>,

    pub symbols: Vec<Point>,

    pub parts: HashSet<Number>,

    pub gears: i32,
}

impl Engine {
    fn build(&mut self, input: &str) {
        for (row, ogline) in input.lines().enumerate() {
            let mut number = String::new();

            let line = ".".to_owned() + ogline + ".";

            for (column, chr) in line.chars().enumerate() {
                if chr.is_numeric() {
                    number.push(chr);
                    continue;
                } else if !chr.is_alphabetic() && chr != '.' {
                    self.symbols.push((row as i32, column as i32, chr));
                }

                if !number.is_empty() {
                    let columns: Vec<i32> =
                        (column - number.len()..column).map(|i| i as i32).collect();

                    self.numbers.push(Number {
                        value: number,
                        row: row as i32,
                        columns,
                    });

                    number = String::new();
                }
            }
        }
    }

    fn scan(&mut self) {
        for (row, column, chr) in &self.symbols {
            let mut gears: HashSet<Number> = HashSet::new();

            for x in -1..=1 {
                for y in -1..=1 {
                    let symrow = row + x;
                    let symcol = column + y;
                    // println!("   {symrow}::{symcol}")

                    self.numbers
                        .iter()
                        .filter(|&n| n.row == symrow && n.columns.contains(&symcol))
                        .for_each(|n| {
                            self.parts.insert(n.clone());
                            if chr == &'*' {
                                gears.insert(n.clone());
                            }
                        });
                }
            }

            println!("{:?}", gears);

            if gears.len() > 1 {
                let result: i32 = gears
                    .iter()
                    .map(|n| n.value.parse::<i32>().unwrap())
                    .product(); // [0] * [1] * [n]

                self.gears += result;
            }
        }
    }
}

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;

    let mut engine = Engine::default();

    engine.build(&input);

    // println!("{:#?}", engine.numbers);
    // println!("{:#?}", engine.symbols);

    engine.scan();

    // println!("{:#?}", engine.parts);
    let mut total = 0;
    for part in engine.parts {
        total += part.value.parse::<i32>().unwrap();
    }

    println!("Total :: {total}");

    println!("Gears :: {}", engine.gears);

    Ok(())
}
