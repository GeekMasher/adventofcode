use crate::Arguments;
use anyhow::Result;

#[derive(Debug, Default)]
struct Card {
    pub id: i32,
    pub winning: Vec<i32>,
    pub numbers: Vec<i32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let (card_number, card_results) =
            value.split_once(": ").expect("Failed to split card string");

        let card_id = card_number
            .replace("Card ", "")
            .trim()
            .parse::<i32>()
            .expect("Failed to parse Card ID");

        let (winning_hard, our_hand) = card_results
            .split_once(" | ")
            .expect("Failed to split hards");

        let winning: Vec<i32> = winning_hard
            .split(' ')
            .filter_map(|w| match w.parse::<i32>() {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect();

        let numbers: Vec<i32> = our_hand
            .split(' ')
            .filter_map(|w| match w.parse::<i32>() {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect();

        Card {
            id: card_id,
            winning,
            numbers,
        }
    }
}

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;
    let mut total = 0;

    for line in input.lines() {
        let card = Card::from(line);

        let win_number = card
            .numbers
            .iter()
            .filter(|&w| card.winning.contains(w))
            .count();

        println!("{:?} ({})", card, win_number);

        let mut score = 0;
        for i in 0..win_number {
            if i == 0 {
                score = 1;
            } else {
                score = score * 2;
            }
        }
        println!("{score}");
        total += score;
    }

    println!("Total :: {total}");

    assert_eq!(total, 13);

    Ok(())
}
