use crate::Arguments;
use anyhow::Result;

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;
    let mut total = 0;

    for line in input.lines() {
        let mut stack: Vec<i32> = Vec::new();

        line.chars()
            .filter(|chr| chr.is_numeric())
            .for_each(|chr| stack.push(chr.to_digit(10).unwrap() as i32));

        // println!("Stack :: {:?}", stack);

        let mut result = String::new();
        result.push_str(&stack.first().unwrap().to_string());
        result.push_str(&stack.last().unwrap().to_string());

        // println!(" >> {result}");

        total += result.parse::<i32>()?;
    }

    println!("Total :: {total}");

    Ok(())
}
