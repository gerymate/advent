use std::{fs::read_to_string, collections::HashSet};

pub fn solve_day() -> i128 {
    let input = read_to_string("../input11.txt").unwrap();
    solve(&input, 999_999)
}

struct Dims {
    width: usize,
    height: usize,
}

struct Point {
    x: usize,
    y: usize,
}

struct SpaceImage {
    image: Vec<Vec<bool>>,
    dims: Dims,
    empty_columns: HashSet<usize>,
    empty_rows: HashSet<usize>,
    expanse_factor: usize,
}

struct Galaxies(Vec<Point>);

impl Galaxies {
    fn build(space_image: &SpaceImage) -> Galaxies {
        let mut coords = Vec::new();
        let mut y_expansions = 0;
        for y in 0..space_image.dims.height {
            if space_image.empty_rows.contains(&y) {
                y_expansions += 1;
            }
            let mut x_expansions = 0;
            for x in 0..space_image.dims.width {
                if space_image.empty_columns.contains(&x) {
                    x_expansions += 1;
                }
                if space_image.image[y][x] {
                    coords.push(Point { x: x + x_expansions * space_image.expanse_factor, y: y + y_expansions * space_image.expanse_factor });
                }
            }
        }
        Galaxies(coords)
    }
    fn shortest_path(p1: &Point, p2: &Point) -> usize {
        if p2.x > p1.x {
            p2.y - p1.y + p2.x - p1.x
        } else {
            p2.y - p1.y + p1.x - p2.x
        }
    }
}

impl SpaceImage {
    fn build(input: &str, expanse_factor: usize) -> SpaceImage {
        let image: Vec<Vec<bool>> = input.lines().map(
            |line| line.chars().map(
                |ch| ch == '#'
            ).collect()
        ).collect();
        let dims = Dims { width: image[0].len(), height: image.len() };
        SpaceImage{
            dims,
            empty_columns: SpaceImage::get_empty_columns(&image).into_iter().collect(),
            empty_rows: SpaceImage::get_empty_rows(&image).into_iter().collect(),
            image,
            expanse_factor
        }
    }

    fn get_empty_columns(image: &Vec<Vec<bool>>) -> Vec<usize> {
        let mut empty_columns: Vec<usize> = Vec::new();
        let cols = image.len();
        for i in 0..cols {
            if image.iter().all(|row| !row[i]) {
                empty_columns.push(i);
            }
        }
        empty_columns
    }

    fn get_empty_rows(image: &[Vec<bool>]) -> Vec<usize> {
        image.iter().enumerate().filter(|(_i, row)| row.iter().all(|val| !val)).map(|(i, _)| i).collect()
    }
}

fn solve(input: &str, expanse_factor: usize) -> i128 {
    let space_image = SpaceImage::build(input, expanse_factor);
    let gxs = Galaxies::build(&space_image);
    let mut s = 0usize;
    for i in 0..gxs.0.len()-1 {
        for j in i+1..gxs.0.len() {
            s += Galaxies::shortest_path(&gxs.0[i], &gxs.0[j]);
        }
    }
    s as i128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input, 1), 374);
    }

    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input, 9), 1030);
    }

    #[test]
    fn test_solve3() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input, 99), 8410);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        test_input
    }
}
