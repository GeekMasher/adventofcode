use anyhow::Result;

mod cli;
mod days;

pub use crate::cli::*;
pub use crate::days::*;

fn main() -> Result<()> {
    let arguments = setup();

    let mut days = Days::new();
    days.register(1, day1::run);

    if arguments.day == 0 {
        days.show_days();
    } else {
        days.run(&arguments)?;
    }

    Ok(())
}
