use std::{fs::read_to_string, collections::HashMap};
use std::cmp::Ordering;

pub fn solve_day_7() -> i32 {
    let input = read_to_string("../input7.txt").unwrap();
    solve2(&input) as i32
}

struct Hands<'a> (Vec<Hand<'a>>);

impl Hands<'_> {
    fn build<'a>(input: &'a str, has_joker: bool) -> Hands<'_>{
        let mut hands = Vec::new();
        input.lines().for_each(|line| hands.push(Hand::build(line, has_joker)));
        Hands(hands)
    }
}

fn card_value1(ch: char) -> u8 {
    match ch {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

// J is now joker
fn card_value2(ch: char) -> u8 {
    let val = card_value1(ch);
    if val == 11 {
        1
    } else {
        val
    }
}

type Cards = HashMap<char, u8>;

#[derive(Eq, Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: i64,
    setup: Cards,
    typ: u8,
    has_joker: bool,
}

impl Hand<'_> {
    pub fn build<'a>(input: &'a str, has_joker: bool) -> Hand<'a> {
        let mut parts = input.trim().split_whitespace();
        let cards = parts.next().unwrap();
        let bid: i64 = parts.next().unwrap().parse().unwrap();
        let mut setup = Cards::new();
        for ch in ['A','K','Q','J','T','9','8','7','6','5','4','3','2'] {
            setup.insert(ch, 0);
        }
        for ch in cards.chars() {
            setup.insert(ch, setup.get(&ch).unwrap() + 1);
        }
        let typ = if has_joker {
            hand_type2(&setup)
        } else { 
            hand_type1(&setup)
        };
        Hand { cards, bid, setup, typ, has_joker }
    }
}

fn hand_type2(setup: &Cards) -> u8 {
    let ht1 = hand_type1(&setup);
    let jokers = *setup.get(&'J').unwrap();
    if ht1 == 6 || jokers == 4 {
        return 6;
    }
    if jokers == 0 {
        return ht1;
    }
    // 1, 2, or 3 jokers:
    if jokers == 1 {
        return match ht1 {
            0 => 1,
            5 => 6,
            _ => ht1 + 2,
        }
    } else if jokers == 2 {
        return match ht1 {
            1 => 3,
            2 => 5,
            _ => 6,
        }
    } else { // jokers == 3
        return match ht1 {
            3 => 5,
            _ => 6,
       }
    }
}

fn hand_type1(setup: &Cards) -> u8 {
    let pairs = setup.values().filter(|&v| v == &2).count();
    if setup.values().any(|v| v == &5) {
        6
    } else if setup.values().any(|v| v == &4) {
        5
    } else if setup.values().any(|v| v == &3) && pairs == 1 {
        4
    } else if setup.values().any(|v| v == &3) {
        3
    } else if pairs == 2 {
        2
    } else if pairs == 1 {
        1
    } else {
        0
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_cmp = self.typ.cmp(&other.typ);
        if type_cmp != Ordering::Equal {
            return type_cmp;
        };
        let this_cards = self.cards.chars();
        let other_cards = other.cards.chars();
        for (a, b) in this_cards.zip(other_cards) {
            let res = if self.has_joker {
                card_value2(a).cmp(&card_value2(b))
            } else {
                card_value1(a).cmp(&card_value1(b))
            };
            if res != Ordering::Equal {
                return res;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn solve(input: &str) -> i64 {
    let Hands(mut hands) = Hands::build(input, false);
    hands.sort();
    let mut s = 0;
    for (i, hand) in hands.iter().enumerate() {
        s+= (i as i64 + 1) * hand.bid;
    }
    s
}

fn solve2(input: &str) -> i64 {
    let Hands(mut hands) = Hands::build(input, true);
    hands.sort();
    let mut s = 0;
    for (i, hand) in hands.iter().enumerate() {
        s+= (i as i64 + 1) * hand.bid;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 6440);
    }

    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), 5905);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        test_input
    }
}
