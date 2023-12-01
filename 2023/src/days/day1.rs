use crate::Arguments;
use anyhow::Result;

const MAPPINGS: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;
    let mut total = 0;

    for line in input.lines() {
        let mut stack: Vec<i32> = Vec::new();

        for (index, chr) in line.chars().enumerate() {
            if chr.is_numeric() {
                stack.push(chr.to_digit(10).unwrap() as i32);
            } else if let Some(rest_of_line) = line.get(index..) {
                for (number_string, number) in MAPPINGS {
                    if rest_of_line.starts_with(number_string) {
                        stack.push(number);
                    }
                }
            }
        }

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
