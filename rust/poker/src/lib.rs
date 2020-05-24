#![allow(clippy::many_single_char_names)]

use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut result = vec![];
    let mut top = Hand::None;
    for hand in hands {
        let cards = Cards::new(hand);
        if result.is_empty() {
            result.push(*hand);
            top = cards.hand();
        } else if top < cards.hand() {
            result.clear();
            result.push(*hand);
            top = cards.hand();
        } else if top == cards.hand() {
            result.push(*hand);
            top = cards.hand();
        }
    }
    Some(result)
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Card {
    pub number: u8,
    pub suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        if self.number == other.number {
            self.suit.cmp(&other.suit)
        } else {
            self.number.cmp(&other.number)
        }
    }
}

impl Card {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"^(?P<number>\d+|A|J|Q|K)(?P<suit>S|C|D|H)$").unwrap();
        let caps = re.captures(input).unwrap();

        let number = match caps.name("number").map(|num| num.as_str()) {
            Some("A") => 14,
            Some("J") => 11,
            Some("Q") => 12,
            Some("K") => 13,
            Some(num) => num.parse().unwrap(),
            _ => panic!(),
        };
        let suit = match caps.name("suit").map(|suit| suit.as_str()) {
            Some("S") => Suit::Spade,
            Some("C") => Suit::Club,
            Some("D") => Suit::Diamond,
            Some("H") => Suit::Heart,
            _ => panic!(),
        };

        Card { number, suit }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct HighCards(Vec<u8>);

impl Ord for HighCards {
    fn cmp(&self, other: &HighCards) -> Ordering {
        for (x, y) in self
            .0
            .iter()
            .sorted()
            .rev()
            .zip_eq(other.0.iter().sorted().rev())
        {
            if x == y {
                continue;
            }

            return x.cmp(&y);
        }

        Ordering::Equal
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Hand {
    None,
    NoPair(HighCards),
    OnePair(u8, HighCards),
    TwoPair(u8, HighCards),
    ThreeCards(u8, HighCards),
    Straight(u8),
    Flush(u8),
    FullHouse(u8, u8),
    FourCards(u8, u8),
    StraightFlush(u8),
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Cards {
    pub cards: Vec<Card>,
}

impl Cards {
    pub fn new(input: &str) -> Self {
        let cards = input
            .split_whitespace()
            .map(|hand| Card::new(hand))
            .sorted_by_key(|hand| hand.number)
            .collect::<Vec<_>>();

        Cards { cards }
    }

    pub fn hand(&self) -> Hand {
        if let Some(hand) = self.is_straight_flush_hand() {
            return hand;
        }

        if let Some(hand) = self.is_four_cards_hand() {
            return hand;
        }

        if let Some(hand) = self.is_fullhouse_hand() {
            return hand;
        }

        if let Some(hand) = self.is_flush_hand() {
            return hand;
        }

        if let Some(hand) = self.is_straight_hand() {
            return hand;
        }

        if let Some(hand) = self.is_three_cards_hand() {
            return hand;
        }

        if let Some(hand) = self.is_two_pair_hand() {
            return hand;
        }

        if let Some(hand) = self.is_one_pair_hand() {
            return hand;
        }

        Hand::NoPair(HighCards(self.numbers()))
    }

    fn numbers(&self) -> Vec<u8> {
        self.cards
            .iter()
            .map(|hand| hand.number)
            .collect::<Vec<_>>()
    }

    fn suits(&self) -> Vec<Suit> {
        self.cards.iter().map(|hand| hand.suit).collect::<Vec<_>>()
    }

    fn is_straight_flush_hand(&self) -> Option<Hand> {
        if self.is_straight() && self.is_flush_hand().is_some() {
            if let Some(rank) = self.numbers().iter().max() {
                return Hand::StraightFlush(*rank).into();
            }
        }

        None
    }

    fn is_straight_hand(&self) -> Option<Hand> {
        if self.is_straight() {
            if let Some(rank) = self.numbers().iter().max() {
                return Hand::Straight(*rank).into();
            }
        }

        if self.numbers()[..] == [2, 3, 4, 5, 14] {
            // 10 J Q K A
            return Hand::Straight(5).into();
        }

        None
    }

    pub fn is_straight(&self) -> bool {
        self.numbers()
            .iter()
            .tuple_windows::<(_, _)>()
            .all(|(x, y)| x + 1 == *y)
    }

    pub fn is_flush_hand(&self) -> Option<Hand> {
        let unique_suits = self.suits().iter().sorted().dedup().count() == 1;

        if unique_suits {
            if let Some(rank) = self.numbers().iter().max() {
                return Hand::Flush(*rank).into();
            }
        }

        None
    }

    pub fn is_fullhouse_hand(&self) -> Option<Hand> {
        match &self.numbers()[..] {
            [a, b, c, d, e] if (a == b && b == c) && (d == e) => Hand::FullHouse(*a, *d).into(),
            [a, b, c, d, e] if (a == b) && (c == d && d == e) => Hand::FullHouse(*c, *a).into(),
            _ => None,
        }
    }

    pub fn is_four_cards_hand(&self) -> Option<Hand> {
        match &self.numbers()[..] {
            [a, b, c, d, e] if (a == b && b == c && c == d) => Hand::FourCards(*a, *e).into(),
            [a, b, c, d, e] if (b == c && c == d && d == e) => Hand::FourCards(*b, *a).into(),
            _ => None,
        }
    }

    pub fn is_three_cards_hand(&self) -> Option<Hand> {
        match &self.numbers()[..] {
            [a, b, c, d, e] if (a == b && b == c) => {
                Hand::ThreeCards(*a, HighCards(vec![*d, *e])).into()
            }
            [a, b, c, d, e] if (b == c && c == d) => {
                Hand::ThreeCards(*b, HighCards(vec![*a, *e])).into()
            }
            [a, b, c, d, e] if (c == d && d == e) => {
                Hand::ThreeCards(*c, HighCards(vec![*a, *b])).into()
            }
            _ => None,
        }
    }

    pub fn is_two_pair_hand(&self) -> Option<Hand> {
        match &self.numbers()[..] {
            [a, b, c, d, e] if (a == b && c == d) => {
                Hand::TwoPair(*c, HighCards(vec![*a, *e])).into()
            }
            [a, b, c, d, e] if (b == c && d == e) => {
                Hand::TwoPair(*d, HighCards(vec![*b, *a])).into()
            }
            [a, b, c, d, e] if (a == b && d == e) => {
                Hand::TwoPair(*d, HighCards(vec![*a, *c])).into()
            }
            _ => None,
        }
    }

    pub fn is_one_pair_hand(&self) -> Option<Hand> {
        match &self.numbers()[..] {
            [a, b, c, d, e] if (a == b) => Hand::OnePair(*a, HighCards(vec![*c, *d, *e])).into(),
            [a, b, c, d, e] if (b == c) => Hand::OnePair(*b, HighCards(vec![*a, *d, *e])).into(),
            [a, b, c, d, e] if (c == d) => Hand::OnePair(*c, HighCards(vec![*a, *b, *e])).into(),
            [a, b, c, d, e] if (d == e) => Hand::OnePair(*d, HighCards(vec![*a, *b, *c])).into(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_card() {
        assert_eq!(
            Card::new("4S"),
            Card {
                number: 4,
                suit: Suit::Spade
            }
        );

        assert_eq!(
            Card::new("5S"),
            Card {
                number: 5,
                suit: Suit::Spade
            }
        );

        assert_eq!(
            Card::new("7H"),
            Card {
                number: 7,
                suit: Suit::Heart
            }
        );

        assert_eq!(
            Card::new("8D"),
            Card {
                number: 8,
                suit: Suit::Diamond
            }
        );

        assert_eq!(
            Card::new("JC"),
            Card {
                number: 11,
                suit: Suit::Club
            }
        );

        assert_eq!(
            Card::new("AC"),
            Card {
                number: 14,
                suit: Suit::Club
            }
        );
    }

    #[test]
    fn test_new_cards() {
        assert_eq!(
            Cards::new("2S 4C 7S 9H 10H").cards,
            vec![
                Card {
                    number: 2,
                    suit: Suit::Spade
                },
                Card {
                    number: 4,
                    suit: Suit::Club
                },
                Card {
                    number: 7,
                    suit: Suit::Spade
                },
                Card {
                    number: 9,
                    suit: Suit::Heart
                },
                Card {
                    number: 10,
                    suit: Suit::Heart
                },
            ]
        );
    }

    #[test]
    fn test_ord_hands() {
        let straight_flush = Cards::new("7C 6C 5C 4C 3C");
        let four_cards = Cards::new("10C 10D 10H 10S 5D");
        let fullhouse = Cards::new("10S 10H 10D 4S 4D");
        let flush = Cards::new("AH QH 10H 5H 3H");
        let straight = Cards::new("8S 7S 6H 5H 4S");
        let three_cards = Cards::new("QS QC QD 5S 3C");
        let two_pair = Cards::new("KH KD 2C 2D JH");
        let one_pair = Cards::new("10C 10S 6S 4H 2H");
        let no_pair = Cards::new("AD 10D 9S 5C 4C");

        let mut hands = vec![
            &flush,
            &four_cards,
            &fullhouse,
            &no_pair,
            &one_pair,
            &straight,
            &straight_flush,
            &three_cards,
            &two_pair,
        ];

        hands.sort_by_key(|h| h.hand());

        assert_eq!(
            hands,
            vec![
                &no_pair,
                &one_pair,
                &two_pair,
                &three_cards,
                &straight,
                &flush,
                &fullhouse,
                &four_cards,
                &straight_flush,
            ]
        )
    }

    #[test]
    fn test_is_straight_flush() {
        // assert!(Hands::new("7C 6C 5C 4C 3C").is_straight_flush());
        // assert!(Hands::new("AH KH QH JH 10H").is_straight_flush());
        // assert!(Hands::new("7H 6H 5H 4H 3H").is_straight_flush());
        // assert!(Hands::new("5S 4S 3S 2S AS").is_straight_flush());
        // assert!(Hands::new("JC 10C 9C 8C 7C").is_straight_flush());
        // assert!(Hands::new("JD 10D 9D 8D 7D").is_straight_flush());
    }

    // #[test]
    // fn test_is_four_cards() {
    //     assert!(Cards::new("10C 10D 10H 10S 5D").is_four_cards());
    //     assert!(Cards::new("6D 6H 6S 6C KS").is_four_cards());
    //     assert!(Cards::new("10C 10D 10H 10S QC").is_four_cards());
    // }

    // #[test]
    // fn test_is_fullhouse() {
    //     assert!(Cards::new("10S 10H 10D 4S 4D").is_fullhouse());
    //     assert!(Cards::new("9H 9C 9S AH AD").is_fullhouse());
    //     assert!(Cards::new("AS AC AD 4D 4C").is_fullhouse());
    //     assert!(Cards::new("AS AC AD 3S 3D").is_fullhouse());
    // }

    // #[test]
    // fn test_is_flush() {
    //     assert!(Cards::new("AH QH 10H 5H 3H").is_flush());
    //     assert!(Cards::new("KS QS JS 9S 6S").is_flush());
    //     assert!(Cards::new("AD KD 7D 6D 2D").is_flush());
    //     assert!(Cards::new("AH QH 10H 5H 3H").is_flush());
    // }

    // #[test]
    // fn test_is_straight() {
    //     assert!(Cards::new("8S 7S 6H 5H 4S").is_straight());
    //     assert!(Cards::new("6D 5S 4D 3H 2C").is_straight());
    //     assert!(Cards::new("8S 7S 6H 5H 4S").is_straight());
    //     assert!(Cards::new("8H 7D 6C 5C 4H").is_straight());
    // }

    // #[test]
    // fn test_is_three_cards() {
    //     assert!(Cards::new("QS QC QD 5S 3C").is_three_cards());
    //     assert!(Cards::new("5C 5H 5D QD 10C").is_three_cards());
    //     assert!(Cards::new("8C 8H 8D AC 2D").is_three_cards());
    //     assert!(Cards::new("8S 8H 8D 5S 3C").is_three_cards());
    // }

    // #[test]
    // fn test_is_two_pair() {
    //     assert!(Cards::new("KH KD 2C 2D JH").is_two_pair());
    //     assert!(Cards::new("JD JS 10S 10C 9S").is_two_pair());
    //     assert!(Cards::new("9C 9D 7D 7S 6H").is_two_pair());
    //     assert!(Cards::new("9H 9S 5H 5D KC").is_two_pair());
    //     assert!(Cards::new("4S 4C 3S 3H KD").is_two_pair());
    //     assert!(Cards::new("4H 4D 3D 3C 10S").is_two_pair());
    // }

    // #[test]
    // fn test_is_one_pair() {
    //     assert!(Cards::new("10C 10S 6S 4H 2H").is_one_pair());
    //     assert!(Cards::new("9H 9C AH QD 10D").is_one_pair());
    //     assert!(Cards::new("2D 2H 8S 5C 4C").is_one_pair());
    //     assert!(Cards::new("2C 2S 8C 5H 3H").is_one_pair());
    // }

    // #[test]
    // fn test_is_high_card() {
    //     assert!(Cards::new("AD10D9S5C4C").is_no_pair());
    //     assert!(Cards::new("KCQDJC8H7H").is_no_pair());
    //     assert!(Cards::new("ACQC7D5H2C").is_no_pair());
    //     assert!(Cards::new("AD10D9S5C4C").is_no_pair());
    // }

    #[test]
    fn test_is_high_card() {
        assert_eq!(
            Ordering::Equal,
            HighCards(vec![1, 2, 3]).cmp(&HighCards(vec![1, 2, 3]))
        );
        assert_eq!(
            Ordering::Greater,
            HighCards(vec![9, 2]).cmp(&HighCards(vec![8, 7]))
        );
        assert_eq!(
            Ordering::Less,
            HighCards(vec![13, 10, 5, 3, 2]).cmp(&HighCards(vec![13, 12, 9, 5, 4]))
        );
    }
}
