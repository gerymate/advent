use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::read_to_string,
};

pub fn solve_day() -> i128 {
    let input = read_to_string("../input12.txt").unwrap();
    solve(&input, true)
}

struct ConditionRecord {
    record: Vec<u8>,
    crc: Vec<u8>,
    cache: HashMap<(usize, usize), usize>,
}

impl ConditionRecord {
    fn build(input: &str, unfold: bool) -> ConditionRecord {
        let (mut fst, mut snd) = input.split_once(' ').unwrap();
        let unfolded_pattern = Self::unfold_pattern(fst);
        let unfolded_crc = Self::unfold_crc(snd);
        if unfold {
            fst = &unfolded_pattern;
            snd = &unfolded_crc;
        }
        let record = fst
            .chars()
            .map(|ch| match ch {
                '.' => 0,
                '#' => 1,
                _ => 2,
            })
            .collect();
        let crc = snd
            .split(',')
            .map(|s| s.parse().unwrap())
            .map(|n| "#".repeat(n))
            .collect::<Vec<_>>()
            .join(".")
            .chars()
            .map(|ch| match ch {
                '.' => 0,
                '#' => 1,
                _ => 2,
            })
            .collect();
        let cache = HashMap::new();
        ConditionRecord { record, crc, cache }
    }

    fn unfold_pattern(pattern: &str) -> String {
        [pattern; 5].join("?")
    }

    fn unfold_crc(crc: &str) -> String {
        [crc; 5].join(",")
    }

    fn count_valid_continous(&mut self, start_rec: usize, start_crc: usize) -> usize {
        let spring = self.record.get(start_rec);
        let crc = self.crc.get(start_crc);
        if let (Some(spring), Some(crc)) = (spring, crc) {
            match (*spring, *crc) {
                (2, 1) => self.count_valid_continous(start_rec + 1, start_crc + 1),
                (1, 1) => self.count_valid_continous(start_rec + 1, start_crc + 1),
                (0, 1) => 0,
                (0, 0) => self.count_valid(start_rec + 1, start_crc + 1),
                (2, 0) => self.count_valid(start_rec + 1, start_crc + 1),
                (1, 0) => 0,
                _ => panic!("This cannot happen"),
            }
        } else {
            self.count_valid(start_rec, start_crc)
        }
    }

    fn count_valid(&mut self, start_rec: usize, start_crc: usize) -> usize {
        // In cache?
        if let Some(variants) = self.cache.get(&(start_rec, start_crc)) {
            //eprint!("{} {} ", start_rec, start_crc);

            //eprintln!(" => {}", *variants);
            return *variants;
        }

        // Not in cache?
        let variants =
            // when there should be no more springs
            if start_crc == self.crc.len() {
                if start_rec == self.record.len() {
                    1
                } else if self.record[start_rec] == 1 {
                    0
                } else {
                    self.count_valid(start_rec + 1, start_crc)
                }
            } else if start_rec < self.record.len() {
                match (&self.record[start_rec], &self.crc[start_crc]) {
                    (1, 1) => self.count_valid_continous(start_rec + 1, start_crc + 1),
                    (0, 0) => self.count_valid(start_rec + 1, start_crc + 1),
                    (0, 1) => self.count_valid(start_rec + 1, start_crc),
                    (1, 0) => 0,
                    (2, 0) => self.count_valid(start_rec + 1, start_crc + 1),
                    (2, 1) => {
                        self.count_valid_continous(start_rec + 1, start_crc + 1)
                        + self.count_valid(start_rec + 1, start_crc)
                    }
                    _ => 0,
                }
            } else {
                0
            };

        self.cache.insert((start_rec, start_crc), variants);
        // eprint!("{} {} ", start_rec, start_crc);
        // edbg(&self.record[start_rec..]);
        // eprint!(" ");
        // edbg(&self.crc[start_crc..]);
        // eprintln!(" => {}", variants);
        variants
    }
}

fn edbg(data: &[u8]) {
    data.iter().for_each(|d| eprint!("{}", d))
}

impl Display for ConditionRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let record: String = self.record.iter().map(|cond| cond.to_string()).collect();
        let crc: String = self.crc.iter().map(|cond| cond.to_string()).collect();
        write!(f, "{:?}  |  {:?}", record, crc)
    }
}

fn solve(input: &str, unfold: bool) -> i128 {
    let mut s = 0;
    for line in input.lines() {
        let mut cr = ConditionRecord::build(line, unfold);
        eprintln!("{}", cr);
        let n = cr.count_valid(0, 0);
        eprintln!("result: {}", n);
        s += n;
    }
    s as i128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input, false), 21);
    }

    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input, true), 525152);
    }

    fn get_test_input() -> &'static str {
        "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    }
}
