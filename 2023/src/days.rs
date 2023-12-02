use std::collections::HashMap;

use crate::Arguments;
use anyhow::Result;

pub mod day0;
pub mod day1;
pub mod day2;

pub type DayFn = dyn Fn(&Arguments) -> Result<()>;

pub struct Days {
    pub days: HashMap<i32, Box<DayFn>>,
}

impl Days {
    pub fn new() -> Self {
        Self {
            days: HashMap::new(),
        }
    }

    pub fn register<D>(&mut self, day: i32, func: D)
    where
        D: Fn(&Arguments) -> Result<()> + 'static,
    {
        self.days.insert(day, Box::new(func));
    }

    pub fn show_days(&self) {
        println!("# Days Available");
        for day in self.days.keys() {
            println!("  Day {}", day);
        }
    }

    pub fn run(&self, arguments: &Arguments) -> Result<()> {
        for (day, day_fn) in &self.days {
            if day == &arguments.day {
                println!("# Day {}!\n", day);
                day_fn(&arguments)?;
            }
        }

        Ok(())
    }
}

pub fn load_input(arguments: &Arguments) -> Result<String> {
    let input = match &arguments.input {
        Some(input) => std::fs::read_to_string(input)?,
        None => {
            return Err(anyhow::anyhow!(
                "No input file specified for day {}",
                arguments.day
            ))
        }
    };

    Ok(input)
}
