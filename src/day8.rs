use std::{fs::read_to_string, collections::{HashMap, HashSet}, str::Lines};
use prime_factorization::Factorization;

pub fn solve_day_8() -> u128 {
    let input = read_to_string("../input8.txt").unwrap();
    solve2(&input)
}

fn solve(input: &str) -> u128 {
    let mut lines = input.lines();
    let instructions = get_instructions_iterator(lines.next().unwrap()).collect::<Vec<usize>>();
    let mut inst = instructions.iter().cycle();
    lines.next();
    let nodes = get_nodes_map(lines);
    let mut current = "AAA";
    let target = "ZZZ";
    let mut steps = 0;
    while current != target {
        let way = nodes.get(current).unwrap();
        current = if inst.next().unwrap() == &1 {
            way.1
        } else {
            way.0
        };
        steps += 1;
    }
    steps
} // 15989

fn solve2(input: &str) -> u128 {
    let mut lines = input.lines();
    let instructions = get_instructions_iterator(lines.next().unwrap()).collect::<Vec<usize>>();
    lines.next();
    let nodes = get_nodes_map(lines);
    let mut currents: Vec<&str> = 
        nodes.keys().filter(|&key| key.chars().last() == Some('A')).map(|&key| key).collect();
    let mut counts: Vec<u128> = vec![0; currents.len()];
    for i in 0..currents.len() {
        let mut inst = instructions.iter().cycle();
        while !currents[i].ends_with('Z') {
            let dir = inst.next().unwrap();
            let ways = nodes.get(currents[i]).unwrap();
            currents[i] = if dir == &1 {
                ways.1
            } else {
                ways.0
            };
            counts[i] += 1;
        }
    }
    dbg!(&counts);

    let mut factorset: HashSet<u128> = HashSet::new();
    for n in counts {
        Factorization::run(n).factors.iter().for_each(|&n| {factorset.insert(n);});
    }
    dbg!(&factorset);
    let steps = factorset.iter().fold(1, |a, b| a * b);
    steps
}
/*
[src/day8.rs:51] &counts = [
    14363,
    18157,
    19783,
    19241,
    15989,
    12737,
]
[src/day8.rs:57] &factorset = {
    47,
    59,
    71,
    271,
    67,
    53,
    73,
}
Solution 64: 1124424219
Solution 128: 13830919117339
*/

fn get_instructions_iterator<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    let iter = input.trim().chars().map(|ch| if ch == 'R' {1} else {0});
    iter
}

fn get_nodes_map<'a>(lines: Lines<'_>) -> HashMap<&str, (&str, &str)> {
    let mut nodes = HashMap::new();
    for line in lines {
        let mut kvs = line.split(" = ");
        let key = kvs.next().unwrap();
        let mut values = kvs.next().unwrap().split(", ");
        let mut v1 = values.next().unwrap().chars();
        v1.next();
        let v1 = v1.as_str();
        let mut v2 = values.next().unwrap().chars();
        v2.next_back();
        let v2 = v2.as_str();
        nodes.insert(key, (v1, v2));
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 2);
    }

    #[test]
    fn test_solve_with_repeat() {
        let test_input = get_test_input2();
        assert_eq!(solve(test_input), 6);
    }


    #[test]
    fn test_solve2() {
        let test_input = get_test_input3();
        assert_eq!(solve2(test_input), 6);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        test_input
    }

    fn get_test_input2() -> &'static str {
        let test_input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        test_input
    }

    fn get_test_input3() -> &'static str {
        let test_input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        test_input
    }



}
