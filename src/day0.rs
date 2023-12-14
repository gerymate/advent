use std::fs::read_to_string;

pub fn solve_day_0() -> i32 {
    let input = read_to_string("../input0.txt").unwrap();
    solve1(&input) as i32
}

fn solve(input: &str) -> i64 {
    1
}

fn solve2(input: &str) -> i64 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), -1);
    }

    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), -2);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
test_input 1 2 3";
        test_input
    }
}
