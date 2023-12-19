use std::fs::read_to_string;

pub fn solve_day() -> i128 {
    let input = read_to_string("../input0.txt").unwrap();
    solve(&input)
}

fn solve(_input: &str) -> i128 {
    1
}

fn solve2(_input: &str) -> i128 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 100);
    }

    #[ignore]
    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), 200);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
TEST_INPUT";
        test_input
    }
}
