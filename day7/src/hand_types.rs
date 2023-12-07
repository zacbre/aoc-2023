use std::cmp::Ordering;
use std::collections::BTreeMap;
use crate::Hand;

pub const high_card_order: &[char; 13] = &['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

pub trait HandType {
    fn matches(&self, hand: &Hand) -> bool;
    fn get_strength(&self) -> usize;
}

pub fn compare_high_card(first_hand: &Hand, second_hand: &Hand) -> Ordering {
    for i in 0..first_hand.cards.len() {
        // get the index of the card from high card order
        let first_index = high_card_order.iter().position(|&c| c == first_hand.cards[i]).unwrap();
        let second_index = high_card_order.iter().position(|&c| c == second_hand.cards[i]).unwrap();
        if first_index < second_index {
            return Ordering::Greater;
        } else if first_index > second_index {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

pub fn sort_hand_types(hands: Vec<Hand>) -> Vec<Hand> {
    let mut hand_types: Vec<Hand> = Vec::new();
    let matchers: Vec<Box<dyn HandType>> = vec![
        Box::new(FiveOfAKind{}),
        Box::new(FourOfAKind{}),
        Box::new(FullHouse{}),
        Box::new(ThreeOfAKind{}),
        Box::new(TwoPair{}),
        Box::new(OnePair{}),
        Box::new(HighCard{}),
    ];
    for mut hand in hands {
        let mut hand_matched = false;
        for matcher in &matchers {
            if matcher.matches(&hand) {
                hand.strength = matcher.get_strength();
                hand_types.push(hand.clone());
                hand_matched = true;
                break;
            }
        }
        if hand_matched == false {
            panic!("No hand type matched for {:?}", hand);
        }
    }
    hand_types.sort();
    hand_types
}

fn cards_to_count(cards: &Vec<char>) -> BTreeMap<char, usize> {
    let mut map = BTreeMap::new();
    for card in cards {
        let count = map.entry(*card).or_insert(0);
        *count += 1;
    }
    map
}

pub struct FiveOfAKind;
impl HandType for FiveOfAKind {
    fn matches(&self, hand: &Hand) -> bool {
        let max_cards = cards_to_count(&hand.cards);
        let (chr, count) = max_cards.iter().max_by_key(|&(c, v)| if c != &'J' { v } else { &0 }).unwrap();
        if chr == &'J' && *count == 5 {
            return true;
        }
        if cfg!(feature = "joker") {
            if max_cards.contains_key(&'J') {
                let j_count = max_cards.get(&'J').unwrap();
                return *count + j_count == 5;
            }
        }
        return *count == 5;
    }
    fn get_strength(&self) -> usize {
        7
    }
}

pub struct FourOfAKind;
impl HandType for FourOfAKind {
    fn matches(&self, hand: &Hand) -> bool {
        let max_cards = cards_to_count(&hand.cards);
        if cfg!(feature = "joker") {
            let (chr, count) = max_cards.iter().max_by_key(|&(c, v)| if c != &'J' { v } else { &0 }).unwrap();
            if max_cards.contains_key(&'J') {
                let j_count = max_cards.get(&'J').unwrap();
                return *count + *j_count == 4;
            }
        }
        let (chr, count) = max_cards.iter().max_by_key(|&(_, v)| v).unwrap();
        println!("{:?} {:?}", chr, count);
        return *count == 4;
    }
    fn get_strength(&self) -> usize {
        6
    }
}

pub struct FullHouse;
impl HandType for FullHouse {
    fn matches(&self, hand: &Hand) -> bool {
        let mut max_cards = cards_to_count(&hand.cards);
        if cfg!(feature = "joker") {
            let (chr, count) = max_cards.iter().max_by_key(|&(c, v)| if c != &'J' { v } else { &0 }).unwrap();
            if max_cards.contains_key(&'J') {
                let j_count = max_cards.get(&'J').unwrap();
                max_cards.insert(*chr, *count + *j_count);
                // get rid of the J key.
                max_cards.remove(&'J');
            }
        }
        if max_cards.len() >= 2 {
            let mut iter = max_cards.iter();
            let (chr1, count1) = iter.next().unwrap();
            let (chr2, count2) = iter.next().unwrap();
            return max_cards.len() == 2 && (*count1 == 3 && *count2 == 2 || *count1 == 2 && *count2 == 3)
        }
        return false;
    }
    fn get_strength(&self) -> usize {
        5
    }
}

pub struct ThreeOfAKind;
impl HandType for ThreeOfAKind {
    fn matches(&self, hand: &Hand) -> bool {
        let max_cards = cards_to_count(&hand.cards);
        let (chr, count) = max_cards.iter().max_by_key(|&(c, v)| if c != &'J' { v } else { &0 }).unwrap();
        if cfg!(feature = "joker") {
            if max_cards.contains_key(&'J') {
                let j_count = max_cards.get(&'J').unwrap();
                return *count + *j_count == 3;
            }
        }
        return *count == 3;
    }
    fn get_strength(&self) -> usize {
        4
    }
}

pub struct TwoPair;
impl HandType for TwoPair {
    fn matches(&self, hand: &Hand) -> bool {
        let mut max_cards = cards_to_count(&hand.cards);
        let (chr, count) = max_cards.iter().max_by_key(|&(c, v)| if c != &'J' { v } else { &0 }).unwrap();
        if cfg!(feature = "joker") {
            if max_cards.contains_key(&'J') {
                let j_count = max_cards.get(&'J').unwrap();
                max_cards.insert(*chr, *count + *j_count);
                max_cards.remove(&'J');
            }
        }
        // find 2 sets of cards that match 2 count
        max_cards.iter().filter(|&(_, v)| *v == 2).count() == 2
    }
    fn get_strength(&self) -> usize {
        3
    }
}

pub struct OnePair;
impl HandType for OnePair {
    fn matches(&self, hand: &Hand) -> bool {
        let mut max_cards = cards_to_count(&hand.cards);
        let (chr, count) = max_cards.iter().max_by_key(|&(c, v)| if c != &'J' { v } else { &0 }).unwrap();
        if cfg!(feature = "joker") {
            if max_cards.contains_key(&'J') {
                let j_count = max_cards.get(&'J').unwrap();
                max_cards.insert(*chr, *count + *j_count);
                max_cards.remove(&'J');
            }
        }
        // find 2 sets of cards that match 2 count
        max_cards.iter().filter(|&(_, v)| *v == 2).count() == 1
    }
    fn get_strength(&self) -> usize {
        2
    }
}

pub struct HighCard;
impl HandType for HighCard {
    fn matches(&self, hand: &Hand) -> bool {
        let mut cards = hand.cards.clone();
        cards.sort();
        cards.dedup();
        cards.len() == 5
    }
    fn get_strength(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod test {
    use crate::hand_types::HandType;

    #[test]
    fn test_five_of_a_kind() {
        use crate::hand_types::FiveOfAKind;
        use crate::Hand;
        let hand = Hand { cards: vec!['A', 'A', 'A', 'A', 'A'], bid: 0, strength: 0 };
        let hand1 = Hand { cards: vec!['Z', 'Z', 'Z', 'Z', 'Z'], bid: 0, strength: 0 };
        let hand4 = Hand { cards: vec!['J', 'J', 'J', 'J', 'J'], bid: 735, strength: 0 };
        assert!(FiveOfAKind{}.matches(&hand));
        assert!(FiveOfAKind{}.matches(&hand1));
        assert!(FiveOfAKind{}.matches(&hand4));

        if cfg!(feature = "joker") {
            let hand2 = Hand { cards: vec!['Z', 'Z', 'J', 'Z', 'Z'], bid: 0, strength: 0 };
            let hand3 = Hand { cards: vec!['J', 'J', 'J', '8', 'J'], bid: 0, strength: 0 };
            assert!(FiveOfAKind{}.matches(&hand2));
            assert!(FiveOfAKind{}.matches(&hand3));
        }
    }

    #[test]
    fn test_four_of_a_kind() {
        use crate::hand_types::FourOfAKind;
        use crate::Hand;

        let hand1 = Hand { cards: vec!['A', 'K', 'A', 'A', 'A'], bid: 0, strength: 0 };
        let hand2 = Hand { cards: vec!['Z', 'K', 'Z', 'Z', 'Z'], bid: 0, strength: 0 };
        let hand3 = Hand { cards: vec!['K', 'K', 'Z', 'Z', 'Z'], bid: 0, strength: 0 };
        assert!(FourOfAKind{}.matches(&hand1));
        assert!(FourOfAKind{}.matches(&hand2));
        assert!(!FourOfAKind{}.matches(&hand3));

        if cfg!(not(feature = "joker")) {
            let hand = Hand { cards: vec!['J', 'J', 'J', '8', 'J'], bid: 0, strength: 0 };
            assert!(FourOfAKind{}.matches(&hand));
        }

        if cfg!(feature = "joker") {
            let hand = Hand { cards: vec!['K', 'J', 'J', 'Z', 'Z'], bid: 0, strength: 0 };
            let hand1 = Hand { cards: vec!['A', 'J', '3', '2', 'J'], bid: 0, strength: 0 };
            assert!(FourOfAKind{}.matches(&hand));
            assert!(!FourOfAKind{}.matches(&hand1));
        }
    }

    #[test]
    fn test_full_house() {
        use crate::hand_types::FullHouse;
        use crate::Hand;
        let hand = Hand { cards: vec!['A', 'K', 'A', 'K', 'A'], bid: 0, strength: 0 };
        let hand2 = Hand { cards: vec!['F', 'Z', 'F', 'Z', 'F'], bid: 0, strength: 0 };
        assert!(FullHouse{}.matches(&hand));
        assert!(FullHouse{}.matches(&hand2));


        if cfg!(feature = "joker") {
            let hand1 = Hand { cards: vec!['Q', 'A', 'Q', 'A', 'J'], bid: 0, strength: 0 };
            let hand3 = Hand { cards: vec!['J', 'Z', 'F', 'F', 'Z'], bid: 0, strength: 0 };
            assert!(FullHouse{}.matches(&hand1));
            assert!(FullHouse{}.matches(&hand3));
        }
    }

    #[test]
    fn test_three_of_a_kind() {
        use crate::hand_types::ThreeOfAKind;
        use crate::Hand;

        let hand1 = Hand { cards: vec!['F', 'Z', 'F', 'A', 'F'], bid: 0, strength: 0 };
        assert!(ThreeOfAKind{}.matches(&hand1));

        if cfg!(feature = "joker") {
            let hand = Hand { cards: vec!['Q', 'Q', 'F', 'J', 'A'], bid: 0, strength: 0 };
            let hand1 = Hand { cards: vec!['F', 'Z', 'J', 'A', 'F'], bid: 0, strength: 0 };
            let hand2 = Hand { cards: vec!['A', 'F', 'Q', 'J', 'A'], bid: 0, strength: 0 };

            assert!(ThreeOfAKind{}.matches(&hand));
            assert!(ThreeOfAKind{}.matches(&hand1));
            assert!(ThreeOfAKind{}.matches(&hand2));
        }
    }

    #[test]
    fn test_two_pair() {
        use crate::hand_types::TwoPair;
        use crate::Hand;
        let hand = Hand { cards: vec!['A', 'K', 'A', 'K', 'Q'], bid: 0, strength: 0 };
        // two pair not possible with J?
        //let hand = Hand { cards: vec!['A', 'J', 'A', 'K', 'Q'], bid: 0, strength: 0 };
        assert!(TwoPair{}.matches(&hand));
    }

    #[test]
    fn test_one_pair() {
        use crate::hand_types::OnePair;
        use crate::Hand;
        let hand = Hand { cards: vec!['A', 'W', 'A', 'K', 'Q'], bid: 0, strength: 0 };
        assert!(OnePair{}.matches(&hand));

        if cfg!(feature = "joker") {
            let hand1 = Hand { cards: vec!['3', '8', '4', 'J', '6'], bid: 0, strength: 0 };
            assert!(OnePair{}.matches(&hand1));
        }

    }

    #[test]
    fn test_high_card() {
        use crate::hand_types::HighCard;
        use crate::Hand;
        let hand = Hand { cards: vec!['A', 'J', 'W', 'K', 'Q'], bid: 0, strength: 0 };
        assert!(HighCard{}.matches(&hand));
    }

    #[test]
    fn test_compare_high_card() {
        use crate::hand_types::compare_high_card;
        use crate::Hand;
        let first_hand = Hand { cards: vec!['A', 'J', 'W', 'K', 'Q'], bid: 0, strength: 0 };
        let second_hand = Hand { cards: vec!['A', 'K', 'Q', 'J', 'Q'], bid: 0, strength: 0 };
        assert_eq!(compare_high_card(&first_hand, &second_hand), std::cmp::Ordering::Less);

        let first_hand = Hand { cards: vec!['A', 'J', 'Q', 'K', 'Q'], bid: 0, strength: 0 };
        let second_hand = Hand { cards: vec!['A', 'J', 'Q', 'K', 'Q'], bid: 0, strength: 0 };
        assert_eq!(compare_high_card(&first_hand, &second_hand), std::cmp::Ordering::Equal);

        let first_hand = Hand { cards: vec!['A', 'K', 'T', 'K', 'Q'], bid: 0, strength: 0 };
        let second_hand = Hand { cards: vec!['A', 'J', 'Q', 'J', 'Q'], bid: 0, strength: 0 };
        assert_eq!(compare_high_card(&first_hand, &second_hand), std::cmp::Ordering::Greater);
    }
}