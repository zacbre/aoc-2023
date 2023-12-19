use std::collections::BTreeMap;

fn main() {
    let input = r"
";

    let cards = parse(input);
    println!("{:?}", cards);
    part_one(&cards);
    part_two(&cards);
}

fn part_one(cards: &BTreeMap<usize, Card>) {
    let mut total_points = 0;
    for (_index, card) in cards {
        total_points += match_card_points(card);
    }
    println!("Total points: {}", total_points);
}

fn match_card_points(card: &Card) -> usize {
    let mut current_card_points = 0;
    for number in &card.winning_numbers {
        if card.our_numbers.contains(number) {
            if current_card_points > 0 {
                current_card_points *= 2;
            } else {
                current_card_points = 1;
            }
        }
    }
    current_card_points
}

fn match_card_total(card: &Card) -> usize {
    let mut current_card_points = 0;
    for number in &card.winning_numbers {
        if card.our_numbers.contains(number) {
            current_card_points += 1;
        }
    }
    current_card_points
}

fn part_two(cards: &BTreeMap<usize, Card>) {
    let mut total_cards = cards.len();
    let mut copies: BTreeMap<usize, usize> = BTreeMap::new();
    // fill the copies map with 1
    copies.append(&mut cards.iter().map(|(index, _)| (*index, 1)).collect::<BTreeMap<usize, usize>>());
    for (index, card) in cards {
        // get copies of current card.
        let card_copies = copies[index];
        let matches = match_card_total(card);
        //println!("Card {} has {} matches and {} copies", index, matches, card_copies);
        // create copies of the next cards.
        for _ in 0..card_copies {
            for i in index+1..index+1+matches {
                //println!("Granting card {} a copy of card {}", index, i);
                // get current copies
                let card_copies = copies.get(&i).unwrap_or(&0);
                copies.insert(i, card_copies+1);
                total_cards += 1;
            }
        }
    }

    println!("{:?}", copies);

    println!("Part Two: {}", total_cards);
    println!("Part Two(2): {}", copies.values().sum::<usize>());
}

#[derive(Debug)]
struct Card {
    pub winning_numbers: Vec<usize>,
    pub our_numbers: Vec<usize>,
}

fn parse(input: &str) -> BTreeMap<usize, Card> {
    let mut cards: BTreeMap<usize, Card> = BTreeMap::new();

    let mut current_card = 0;
    for line in input.lines() {
        if line.len() == 0 { continue; }

        let mut card = Card {
            winning_numbers: Vec::new(),
            our_numbers: Vec::new(),
        };

        let line = &line[line.find(": ").unwrap()+2..line.len()];
        for number_line in line.split(" | ") {
            let numbers: Vec<usize> = number_line.trim().replace("  ", " ").split(" ").map(|n| n.trim().parse().unwrap()).collect();
            if card.winning_numbers.len() == 0 {
                card.winning_numbers = numbers;
            } else {
                card.our_numbers = numbers;
            }
        }
        cards.insert(current_card, card);
        current_card += 1;
    }

    cards
}
