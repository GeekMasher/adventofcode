use crate::Arguments;
use anyhow::Result;

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;

    println!("Input:\n{}", input);

    Ok(())
}
