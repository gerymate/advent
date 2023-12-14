use std::fs::read_to_string;
use std::collections::{HashSet};
use regex::Regex;
use once_cell::sync::Lazy;

pub fn solve_day_4() -> i32 {
    let input = read_to_string("../input4.txt").unwrap();
    solve2(&input)
}

#[derive(Debug)]
struct Game {
    card_no: usize,
    winning_numbers: HashSet<u8>,
    my_numbers: Vec<u8>
}

impl Game {
    pub fn build(desc: &str) -> Option<Game> {
        static RE: Lazy<Regex> = Lazy::new(|| 
            Regex::new(r"^Card\s+([0-9]+):\s+((?:[0-9]+\s*)+)\s+\|\s+((?:[0-9]+\s*)+)$").unwrap());
        let caps = RE.captures(desc)?;
        let card_no = caps.get(1)?.as_str().parse::<usize>().ok()?;
        let winning_numbers = caps.get(2)?.as_str().trim().split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect::<HashSet<u8>>();
        let my_numbers = caps.get(3)?.as_str().trim().split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        Some(Game { card_no, winning_numbers, my_numbers })
    }

    pub fn score(&self) -> i32 {
        let hit = self.my_numbers.iter().filter(|num| self.winning_numbers.contains(num)).count() as u32;
        if hit == 0 {
            0
        } else {
            2i32.pow(hit - 1)
        }
    }

    pub fn count_wins(&self) -> usize {
        self.my_numbers.iter().filter(|num| self.winning_numbers.contains(num)).count()
    }
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(Game::build)
        .map(Option::unwrap)
        .map(|game| game.score())
        .sum()
}

fn solve2(input: &str) -> i32 {
    let games = input.lines().map(Game::build).map(Option::unwrap);
    let mut card_numbers = vec![1; games.clone().count()];
    for (i, game) in games.enumerate() {
        let n = game.count_wins();
        for j in i+1..i+1+n {
            card_numbers[j] += card_numbers[i];
        }
    }
    card_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_build() {
        let test_input = "Card 1: 41 48  6 86 17 | 83 86  6 31 17  9 48 53";
        let game = Game::build(test_input).unwrap();
        assert_eq!(game.card_no, 1);
        assert_eq!(game.my_numbers.len(), 8);
        assert_eq!(game.my_numbers[0], 83);
        assert_eq!(game.my_numbers[7], 53);
        assert_eq!(game.winning_numbers.len(), 5);
        assert!(game.winning_numbers.contains(&41));
        assert!(game.winning_numbers.contains(&17));
    }

    // skip next test for now
    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 13);
    }

    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), 30);
    }
 
    fn get_test_input() -> &'static str {
      let test_input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
      test_input
    }
}
