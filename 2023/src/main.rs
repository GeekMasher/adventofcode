use anyhow::Result;

mod cli;
mod days;

pub use crate::cli::*;
pub use crate::days::*;

fn main() -> Result<()> {
    let arguments = setup();

    let mut days = Days::new();
    days.register(1, day1::run);
    days.register(2, day2::run);
    days.register(3, day3::run);
    days.register(4, day4::run);

    if arguments.day == 0 {
        days.show_days();
    } else {
        days.run(&arguments)?;
    }

    Ok(())
}
