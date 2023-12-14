use regex::Regex;
use std::fs::read_to_string;

pub fn solve_day_2() -> i32 {
    // read the input file into a string
    let input = read_to_string("../input2.txt").unwrap();
    solve2(&input)
}

const MAX_R: i32 = 12;
const MAX_G: i32 = 13;
const MAX_B: i32 = 14; 

fn solve(input: &str) -> i32 {
    let mut s: i32 = 0;
    for line in input.lines() {
        let re = Regex::new(r"Game (\d+): (.*)").unwrap();
        let caps = re.captures(line).unwrap();
        let game_number = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let game_data = caps.get(2).unwrap().as_str();

        let game_data_list: Vec<&str> = game_data.split(";").collect();
        let mut valid_game = true;
        for round in game_data_list {
            let round_data_list: Vec<&str> = round.split(",").map(|s| s.trim()).collect();
            for round_data in round_data_list {
                let round_data_list: Vec<&str> = round_data.split(" ").collect();
                if round_data_list.len() != 2 {
                    eprintln!("Error: {} is not a valid round", round_data);
                    continue;
                }
                let color = round_data_list[1];
                let number = round_data_list[0].parse::<i32>().unwrap();
                // check the color and number against the max values
                match color {
                    "red" => {
                        if number > MAX_R {
                            //println!("Game {}: {} is greater than {}", game_number, number, MAX_R);
                            valid_game = false;
                            break;
                        }
                    },
                    "green" => {
                        if number > MAX_G {
                            //println!("Game {}: {} is greater than {}", game_number, number, MAX_G);
                            valid_game = false;
                            break;
                        }
                    },
                    "blue" => {
                        if number > MAX_B {
                            //println!("Game {}: {} is greater than {}", game_number, number, MAX_B);
                            valid_game = false;
                            break;
                        }
                    },
                    _ => println!("Error: {} is not a valid color", color),
                }
            }
        }
        if valid_game {
            s += game_number;
        }        
    };
    s
}

fn solve2(input: &str) -> i32 {
    let mut s: i32 = 0;
    for line in input.lines() {
        let re = Regex::new(r"Game (\d+): (.*)").unwrap();
        let caps = re.captures(line).unwrap();
        let game_data = caps.get(2).unwrap().as_str();

        let game_data_list: Vec<&str> = game_data.split(";").collect();
        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        for round in game_data_list {
            let round_data_list: Vec<&str> = round.split(",").map(|s| s.trim()).collect();
            for round_data in round_data_list {
                let cube_info_list: Vec<&str> = round_data.split(" ").collect();
                if cube_info_list.len() != 2 {
                    eprintln!("Error: {} is not a valid cube info", round_data);
                    continue;
                }
                let color = cube_info_list[1];
                let number = cube_info_list[0].parse::<i32>().unwrap();
                // check the color and number against the max values
                match color {
                    "red" => {
                        if number > max_r {
                            max_r = number;
                        }
                    },
                    "green" => {
                        if number > max_g {
                            max_g = number;
                        }
                    },
                    "blue" => {
                        if number > max_b {
                            max_b = number;
                        }
                    },
                    _ => println!("Error: {} is not a valid color", color),
                }
            }
        }
        s += max_b * max_g * max_r;
    };
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve(test_input), 8);
    }

    #[test]
    fn test_solve_2() {
        let test_input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve2(test_input), 2286);
    }
}
