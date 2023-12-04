use crate::Arguments;
use anyhow::Result;

#[derive(Debug, Default)]
struct Card {
    pub id: i32,
    pub winning: Vec<i32>,
    pub numbers: Vec<i32>,
    pub score: i32,
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

        let score = numbers.iter().filter(|&w| winning.contains(w)).count();

        Card {
            id: card_id,
            winning,
            numbers,
            score: score as i32,
        }
    }
}

fn card_counter(counter: &mut i32, cards: &Vec<Card>, current: &Card) {
    *counter += 1;
    for i in 0..current.score {
        let offset = current.id + i;
        let card = cards.get(offset as usize).unwrap();
        card_counter(counter, cards, card)
    }
}

pub fn run(arguments: &Arguments) -> Result<()> {
    let input = super::load_input(arguments)?;
    let mut total = 0;

    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let card = Card::from(line);

        cards.push(card);
    }

    for card in &cards {
        println!("{:?}", card);
        card_counter(&mut total, &cards, card);
    }

    println!("Total :: {total}");

    assert_eq!(total, 13);

    Ok(())
}
