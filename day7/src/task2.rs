use std::fs;
use std::cmp::Ordering;

pub fn execute() {
    let binding = fs::read_to_string("txts/input.txt")
                            .expect("Error opening file");
    let contents: Vec<_> = binding
                            .lines()
                            .collect();
    
    let mut decks: Vec<Deck> = Vec::new();

    for (i, line) in contents.iter().enumerate() {
        let current: Deck = Deck::new(line);
        //println!("{} {:#?}", i + 1, current.get_state());
        decks.push(current);
    }

    decks.sort_by(|a, b| {
        if a.get_state().get_numeric() == b.get_state().get_numeric() {
            for i in 0..a.cards.len() {
                if a.cards[i].strength != b.cards[i].strength {
                    return a.cards[i].strength.cmp(&b.cards[i].strength);
                }
            }
            return Ordering::Equal;
        }
        else {
            a.get_state().get_numeric().cmp(&b.get_state().get_numeric())
        }
    });

    let mut answer: i32 = 0;

    for i in 0..decks.len() {
        answer += decks[i].bid * (i as i32 + 1);
    }

    println!("{}", answer);
}

use std::collections::HashMap;

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
    present: HashMap<i32, i32>,
    num_count: HashMap<i32, i32>,
    bid: i32
}

impl Deck {
    fn new(input: &str) -> Deck {
        let strings: Vec<_> = input.split_whitespace().collect();
        let string = strings[0];
        let bid: i32 = strings[1].trim().parse().unwrap();


        let mut cards: Vec<Card> = Vec::new();
        let mut present: HashMap<i32, i32> = HashMap::new();
        let mut num_count: HashMap<i32, i32> = HashMap::new();

        for i in 0..5 {
            let current_char = string.chars().nth(i).unwrap();

            let new_card = Card::new(current_char);
            let strong = new_card.strength;

            cards.push(new_card);
            *present.entry(strong).or_insert(0) += 1;
        }

        for (key, value) in &present {
            if *key != 1 {
                *num_count.entry(*value).or_insert(0) += 1;
            }
        }

        Deck {cards, present, num_count, bid}
    }

    fn get_state(&self) -> States {
        if let Some(tmp) = self.num_count.get(&5) {
            return States::FiveSame;
        }
        if let Some(tmp) = self.num_count.get(&4) {
            if let Some(tmp2) = self.present.get(&1) {
                return States::FiveSame;
            }
            return States::FourSame;
        }
        if let Some(tmp) = self.num_count.get(&3) {
            if let Some(tmp2) = self.num_count.get(&2) {
                return States::FullHouse;
            }
            return match self.present.get(&1) {
                Some(2) => States::FiveSame,
                Some(1) => States::FourSame,
                _ => States::ThreeSame
            }
        }
        if let Some(tmp) = self.num_count.get(&2) {
            if *tmp == 2 {
                return match self.present.get(&1) {
                    Some(1) => States::FullHouse,
                    _ => States::TwoPair
                }
            }
            return match self.present.get(&1) {
                Some(3) => States::FiveSame,
                Some(2) => States::FourSame,
                Some(1) => States::ThreeSame,
                _ => States::OnePair
            }
        }

        match self.present.get(&1) {
            Some(5) | Some(4) => States::FiveSame,
            Some(3) => States::FourSame,
            Some(2) => States::ThreeSame,
            Some(1) => States::OnePair,
            _ => States::HighCard
        }
    }
}

#[derive(Debug)]
enum States{
    FiveSame,
    FourSame,
    FullHouse,
    ThreeSame,
    TwoPair,
    OnePair,
    HighCard,
}

impl States {
    fn get_numeric(&self) -> i32 {
        match self {
            States::FiveSame => 7,
            States::FourSame => 6,
            States::FullHouse => 5,
            States::ThreeSame => 4,
            States::TwoPair => 3,
            States::OnePair => 2,
            States::HighCard => 1
        }
    }
}

#[derive(Debug)]
struct Card {
    strength: i32,
}

impl Card {
    fn new(character: char) -> Card {
        let result = match character {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            'J' => 1,
            number => number.to_digit(10).unwrap(),
        } as i32;

        Card {strength: result}
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.strength == other.strength
    }
}