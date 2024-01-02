use std::{fs::read_to_string, fmt::{Formatter, self, Display}};

pub fn solve_day() -> i128 {
    let input = read_to_string("../input12.txt").unwrap();
    solve(&input, true)
}

struct ConditionRecord {
    base: Vec<bool>,
    mask: Vec<bool>,
    x: Vec<bool>,
    damages: Vec<usize>,
    questions: usize,
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
        let base: Vec<bool> = fst.chars().map(|ch| ch == '#').collect();
        let mask: Vec<bool> = fst.chars().map(|ch| ch == '?').collect();
        let questions = mask.iter().filter(|&p| *p).count();
        let x = vec![false; questions];
        let damages: Vec<usize> = snd.split(',').filter_map(|s| s.parse().ok()).collect();
        ConditionRecord {
            base,
            mask,
            x,
            damages,
            questions,
        }
    }

    fn unfold_pattern(pattern: &str) -> String {
        [pattern;5].join("?")
    }

    fn unfold_crc(crc: &str) -> String {
        [crc;5].join(",")
    }

    fn count_valid(&mut self) -> usize {
        let mut s = 0;
        for u in 0..2_usize.pow(self.questions as u32) {
            self.x = vec![false; self.questions];
            for i in 0..self.x.len() {
                if u >> i & 1 == 1 {
                    self.x[i] = true;
                }
            }
            let valid = self.is_valid();
            // eprintln!("{}: {} valid: {}", u, self, valid);
            if valid {
                s += 1;
            }
        }
        s
    }

    fn compile(&self) -> Vec<bool> {
        let mut out = self.base.clone();
        let mut j = 0;
        for (i, &m) in self.mask.iter().enumerate() {
            if m {
                out[i] = self.x[j];
                j += 1;
            }
        }
        out
    }

    fn is_valid(&self) -> bool {
        let candidate = self.compile();
        let mut i = 0;
        for damage_length in &self.damages {
            while i < candidate.len() && !candidate[i] {
                i += 1;
            }
            let mut j = 0;
            while i < candidate.len() && candidate[i] {
                i += 1;
                j += 1;
            }
            if j != *damage_length {
                return false;
            }
        }
        while i < candidate.len() {
            if candidate[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl Display for ConditionRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut j = 0;
        for i in 0..self.base.len() {
            if self.mask[i] {
                if self.x[j] {
                    s.push('X');
                } else {
                    s.push('o');
                }
                j += 1;
            } else {
                if self.base[i] {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
        }
        s.push(' ');
        for &d in &self.damages {
            s.push_str(&format!("{},", d));
        }
        write!(f, "{} ?s: {} 2^q: {}", s, self.questions, 2_usize.pow(self.questions as u32))
    }
}

fn solve(input: &str, unfold: bool) -> i128 {
    let mut s = 0;
    for line in input.lines() {
        let mut cr = ConditionRecord::build(line, unfold);
        eprintln!("{}", cr);
        let n = cr.count_valid();
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
        let test_input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        test_input
    }
}
