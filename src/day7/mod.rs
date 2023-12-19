#![feature(slice_group_by)]
use std::cmp::Ordering;
use hand_types::compare_high_card;

mod input;
mod hand_types;

fn main() {
    let hands = parse(input::input);
    //println!("{:?}", hands);
    let results = hand_types::sort_hand_types(hands);
    for result in &results {
        println!("{:?}", result);
    }
    let mut total = 0;
    let mut index = 1;
    for result in results {
        //print!("{} * {} + ", result.bid, index);
        total += result.bid * index;
        index += 1;
    }
    //println!("");
    println!("{}", total);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    strength: usize
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength > other.strength {
            return Ordering::Greater;
        } else if self.strength < other.strength {
            return Ordering::Less;
        }
        compare_high_card(self, other)
    }
}

fn parse(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let mut cards = Vec::new();
        let mut cards_split = line.split(' ');
        cards_split.next().unwrap().chars().for_each(|c| cards.push(c));
        let bid = cards_split.next().unwrap().parse::<usize>().unwrap();
        hands.push(Hand { cards, bid, strength: 0 });
    }
    hands
}
