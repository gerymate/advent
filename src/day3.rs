use std::fs::read_to_string;

pub fn solve_day_3() -> i32 {
    // read the input file into a string
    let input = read_to_string("../input3.txt").unwrap();
    solve2(&input)
}

#[derive(Debug)]
struct PartNumber {
    number: i32,
    x: i32,
    y: i32,
    length: i32,
}

fn convert_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }
    matrix
}

fn get_symbol_or_dot(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if x < 0 || y < 0 {
        return '.';
    }
    if x >= matrix[0].len() as i32 || y >= matrix.len() as i32 {
        return '.';
    }
    matrix[y as usize][x as usize]
}

fn has_symbol_neighbour(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let sym = get_symbol_or_dot(matrix, x + i, y + j);
            if sym != '.' && !sym.is_numeric() {
                return true;
            }
        }
    }
    false
}

impl PartNumber {
    fn has_symbol_neighbour(&self, matrix: &Vec<Vec<char>>) -> bool {
        for i in self.x..self.x + self.length {
            if has_symbol_neighbour(matrix, i, self.y) {
                return true;
            }
        }
        false
    }

    fn is_neighbour_of(&self, x: i32, y: i32) -> bool {
        let is_to_the_left = self.x == x + 1 && self.y == y;
        let is_to_the_right = self.x + self.length == x && self.y == y;
        if is_to_the_left || is_to_the_right {
            return true;
        }
        if y == self.y + 1 || y == self.y - 1 {
            if x >= self.x - 1 && x <= self.x + self.length {
                return true;
            }
        }
        false
    }
}

struct Gear {
    x: i32,
    y: i32,
    ratios: Vec<i32>,
}

fn get_all_gears(matrix: &Vec<Vec<char>>, part_numbers: &Vec<PartNumber>) -> Vec<Gear> {
    let mut gears: Vec<Gear> = Vec::new();
    let width = matrix[0].len();
    for y in 0..matrix.len() {
        for x in 0..width {
            let c = matrix[y][x];
            if c == '*' {
                let mut ratios: Vec<i32> = Vec::new();
                for part_number in part_numbers {
                    if part_number.is_neighbour_of(x as i32, y as i32) {
                        ratios.push(part_number.number);
                    }
                }
                if ratios.len() == 2 {
                    let gear = Gear {
                        x: x as i32,
                        y: y as i32,
                        ratios: ratios,
                    };
                    gears.push(gear);
                }
            }
        }
    }
    gears
}
                
fn solve2(input: &str) -> i32 {
    let matrix = convert_input_to_matrix(input);
    let part_numbers = get_part_numbers(&matrix);
    let gears = get_all_gears(&matrix, &part_numbers);
    let mut s: i32 = 0;
    for gear in gears {
        s += gear.ratios[0] * gear.ratios[1];
    }
    s
}

fn solve(input: &str) -> i32 {
    let matrix = convert_input_to_matrix(input);
    let part_numbers = get_part_numbers(&matrix);
    let mut s: i32 = 0;
    for part_number in part_numbers {
        if part_number.has_symbol_neighbour(&matrix) {
            s += part_number.number;
        }
    }
    s
}

fn get_number_from_vec(vec: &Vec<char>, start: usize) -> (i32, usize) {
    let mut number = vec[start].to_digit(10).unwrap() as i32;
    let mut i = start + 1;
    while i < vec.len() && vec[i].is_numeric() {
        number = 10 * number + vec[i].to_digit(10).unwrap() as i32;
        i += 1;
    }
    (number, i-start)
}

fn get_part_numbers(matrix: &Vec<Vec<char>>) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut part_number: PartNumber;
    let width = matrix[0].len();
    for y in 0..matrix.len() {
        let mut x = 0;
        while x < width {
            let c = matrix[y][x];
            if c.is_numeric() {
                let (number, length) = get_number_from_vec(&matrix[y], x);
                part_number = PartNumber {
                    number: number,
                    x: x as i32,
                    y: y as i32,
                    length: length as i32,
                };
                part_numbers.push(part_number);
                x += length as usize;
            } else {
                x += 1;
            }
        }
    }
    
    part_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_number_is_neighbour_of() {
        let test_input = get_test_input();
        let mtx = convert_input_to_matrix(test_input);
        let part_numbers = get_part_numbers(&mtx);
        let part_number = &part_numbers[2];
        let not_neighbours = vec![(0, 1), (0, 2), (0, 3), (5, 1), (5, 2), (5, 3)];
        for (x, y) in not_neighbours {
            assert_eq!(part_number.is_neighbour_of(x, y), false);
        }
        let neighbours = vec![(1, 1), (1, 2), (1, 3), (4, 1), (4, 2), (4, 3)];
        for (x, y) in neighbours {
            assert_eq!(part_number.is_neighbour_of(x, y), true);
        }
    }

    #[test]
    fn test_get_number_from_vec() {
        let test_input = get_test_input();
        let mtx = convert_input_to_matrix(test_input);
        let input = &mtx[0];
        let (number, length) = get_number_from_vec(input, 0);
        assert_eq!(number, 467);
        assert_eq!(length, 3);
        let (number, length) = get_number_from_vec(input, 5);
        assert_eq!(number, 114);
        assert_eq!(length, 3);
    }

    #[test]
    fn test_get_part_numbers() {
        let test_input = get_test_input();
        let mtx = convert_input_to_matrix(test_input);
        let part_numbers = get_part_numbers(&mtx);
        assert_eq!(part_numbers.len(), 10);
        assert_eq!(part_numbers[0].number, 467);
        assert_eq!(part_numbers[0].x, 0);
        assert_eq!(part_numbers[0].y, 0);
        assert_eq!(part_numbers[0].length, 3);
        assert_eq!(part_numbers[5].number, 58);
        assert_eq!(part_numbers[5].x, 7);
        assert_eq!(part_numbers[5].y, 5);
        assert_eq!(part_numbers[5].length, 2);
    }

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 4361);
    }

    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), 467835);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        test_input
    }
}
