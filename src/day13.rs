use std::fs::read_to_string;

pub fn solve_day() -> i128 {
    let input = read_to_string("../input13.txt").unwrap();
    solve2(&input)
}

struct Land {
    width: usize,
    height: usize,
    pattern: Vec<Vec<bool>>,
}

impl Land {
    const SKIPVALUE: i128 = -1_000_000;
    fn build(input: &str) -> Land {
        Land {
            width: input.lines().next().unwrap().len(),
            height: input.lines().count(),
            pattern: input
                .lines()
                .map(|line| line.chars().map(|ch| matches!(ch, '#')).collect())
                .collect(),
        }
    }
    fn get_mirror_val(&self, skipif: i128) -> i128 {
        // vertical mirror |
        'xlabel: for x in 0..self.width - 1 {
            // eprintln!("self.width: {} / 2 = {}", self.width, self.width / 2);
            let checks = if x < self.width / 2 {
                x + 1
            } else {
                self.width - x - 1
            };
            for i in 0..checks {
                for y in 0..self.height {
                    // eprintln!("x:{x} y:{y} i:{i}");
                    if self.pattern[y][x - i] != self.pattern[y][x + 1 + i] {
                        continue 'xlabel;
                    }
                }
            }
            let result = x as i128 + 1;
            if result != skipif {
                return result;
            };
        }
        //
        // horizontal mirror |
        'ylabel: for y in 0..self.height - 1 {
            // eprintln!("self.width: {} / 2 = {}", self.width, self.width / 2);
            let checks = if y < self.height / 2 {
                y + 1
            } else {
                self.height - y - 1
            };
            for i in 0..checks {
                for x in 0..self.width {
                    // eprintln!("x:{x} y:{y} i:{i}");
                    if self.pattern[y - i][x] != self.pattern[y + 1 + i][x] {
                        continue 'ylabel;
                    }
                }
            }
            let result = 100 * (y as i128 + 1);
            if result != skipif {
                return result;
            };
        }
        Land::SKIPVALUE
    }
}

fn solve(input: &str) -> i128 {
    input
        .split("\n\n")
        .inspect(|s| eprintln!("Number of blocks: {}", s.len()))
        .map(|block| {
            let land = Land::build(block);
            eprintln!(
                "Block width: {} height: {} size: {}",
                land.width,
                land.height,
                land.width * land.height
            );
            land.get_mirror_val(Land::SKIPVALUE)
        })
        .sum()
}

fn solve2(input: &str) -> i128 {
    input
        .split("\n\n")
        .inspect(|s| eprintln!("Number of blocks: {}", s.len()))
        .map(|block| {
            let mut land = Land::build(block);
            eprintln!(
                "Block width: {} height: {} size: {}",
                land.width,
                land.height,
                land.width * land.height
            );
            let orig_result = land.get_mirror_val(Land::SKIPVALUE);
            eprintln!("orig result: {}", orig_result);
            let mut result = -2_000_000;
            'done: for x in 0..land.width {
                for y in 0..land.height {
                    land.pattern[y][x] = !land.pattern[y][x];
                    result = land.get_mirror_val(orig_result);
                    if result != Land::SKIPVALUE {
                        eprintln!("Result: {}", result);
                        break 'done;
                    }
                    land.pattern[y][x] = !land.pattern[y][x];
                }
            }
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 405);
    }

    #[ignore]
    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), 400);
    }

    fn get_test_input() -> &'static str {
        "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    }
}
