use std::fs::read_to_string;

pub fn solve_day_5() -> i32 {
    let input = read_to_string("../input5.txt").unwrap();
    solve2(&input) as i32
}

fn get_seeds_and_mappings(input: &str) -> (Vec<i64>, Mappings) {
    let mut parts = input.lines();
    // seeds format: "seeds: 79 14 55 13 "
    let seeds = parts.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<i64>()).filter(|o| o.is_ok()).map(Result::unwrap).collect::<Vec<i64>>();

    let mappings = Mappings::build(&parts.collect::<Vec<&str>>().join("\n"));
    (seeds, mappings)
}

fn solve(input: &str) -> i64 {
    let (seeds, mappings) = get_seeds_and_mappings(input);
    seeds.iter().map(|&seed| mappings.apply(seed)).inspect(|v| println!("value: {v}")).min().unwrap()
}

#[derive(Debug)]
struct SeedIterator {
    start: i64,
    length: i64,
    current: i64,
}

impl SeedIterator {
    fn new(start: i64, length: i64) -> SeedIterator {
        SeedIterator {
            start,
            length,
            current: start,
        }
    }
}

impl Iterator for SeedIterator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.current < self.start + self.length {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn solve2(input: &str) -> i64 {
    let (seed_ranges, mappings) = get_seeds_and_mappings(input);
    let mut seed_iterators = Vec::new();
    // loop seed_ranges in pairs
    for seed_range in seed_ranges.chunks(2) {
        seed_iterators.push(SeedIterator::new(seed_range[0], seed_range[1]));
    }
    seed_iterators.into_iter()
        .inspect(|se| println!("se: {:?}", se))
        .map(|seed_iterator| 
            seed_iterator
                .map(|seed| mappings.apply(seed))
                .min().unwrap()
        )
        .min().unwrap()
}

struct Mappings {
    mappings: Vec<Mapping>,
}

impl Mappings {
    fn build(input: &str) -> Mappings {
        let mut mappings = Vec::new();

        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            if line.ends_with("map:") {
                let mut mapping_lines = Vec::new();
                while let Some(mapping_line) = lines.next() {
                    if mapping_line.is_empty() {
                        break;
                    }
                    mapping_lines.push(mapping_line);
                }
                mappings.push(Mapping::build(&mapping_lines.join("\n")));
            }
        }

        Mappings {
            mappings,
        }
    }

    fn apply(&self, value: i64) -> i64 {
        let mut result = value;
        for mapping in &self.mappings {
            result = mapping.apply(result);
        }
        result
    }
}

struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn build(input: &str) -> Mapping {
        let mut ranges = Vec::new();
        for line in input.lines() {
            ranges.push(MappingRange::build(line));
        }
        Mapping {
            ranges,
        }
    }

    fn apply(&self, value: i64) -> i64 {
        for range in &self.ranges {
            match range.apply(value) {
                Some(v) => return v,
                None => continue,
            }
        }
        value
    }
}

struct MappingRange {
    start: i64,
    length: i64,
    delta: i64,
}

impl MappingRange {
    fn build(line: &str) -> MappingRange {
        let mut parts = line.trim().split_whitespace();
        let target = parts.next().unwrap().parse::<i64>().unwrap();
        let start = parts.next().unwrap().parse::<i64>().unwrap();
        let length = parts.next().unwrap().parse::<i64>().unwrap();
        let delta = target - start;
        MappingRange {
            start,
            length,
            delta,
        }
    }

    fn apply(&self, value: i64) -> Option<i64> {
        if value >= self.start && value < self.start + self.length {
            Some(value + self.delta)
        } else {
            None
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 35);
    }

    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), 46);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        test_input
    }
}
