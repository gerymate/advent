use std::fs::read_to_string;

pub fn solve_day() -> i128 {
    let input = read_to_string("../input9.txt").unwrap();
    solve2(&input)
}

struct Sequence {
    series: Vec<Vec<i128>>,
}

impl Sequence {
    pub fn build(line: &str) -> Sequence {
        let mut series = Vec::new();
        let main_series = line.split_whitespace().map(
            |num_str| num_str.parse().unwrap()
        ).collect();
        series.push(main_series);
        Sequence { series }
    }

    pub fn predict(&mut self) -> (i128, i128) {
        let mut i = 0;
        while !all_same(self.series[i].iter()) {
            let parent = &self.series[i];
            let diffs = parent.windows(2).map(|w| w[1] - w[0]).collect();
            self.series.push(diffs);
            i += 1;
        }
        let mut pred_last = 0;
        let mut pred_first = 0;
        while i != 0 {
            pred_last += self.series[i].last().unwrap();
            pred_first = self.series[i].first().unwrap() - pred_first;
            i -= 1;
        }
        (            
            self.series[i].first().unwrap() - pred_first,
            pred_last + self.series[i].last().unwrap()
        )
    }
}

fn all_same<'a>(mut series: impl Iterator<Item = &'a i128>) -> bool {
    match series.next() {
        Some(first) => series.all(|n| n == first),
        None => false,
    }
}

fn solve(input: &str) -> i128 {
    input.lines().map(
        Sequence::build
    ).map(
        |mut s| {let (_, l) = s.predict(); l}
    ).sum()
}

fn solve2(input: &str) -> i128 {
    input.lines().map(
        Sequence::build
    ).map(
        |mut s| {let (f, _) = s.predict(); f}
    ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 114);
    }

    
    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), 2);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        test_input
    }
}
