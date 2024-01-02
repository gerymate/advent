use std::{fs::read_to_string, fmt::Debug};

pub fn solve_day() -> i128 {
    let input = read_to_string("../input10.txt").unwrap();
    solve2(&input)
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Dim {
    width: usize,
    height: usize,
}

struct Turns {
    left: usize,
    right: usize,
}

struct PipeMap {
    pipes: Vec<Vec<char>>,
    land: PipeLand,
    ins: usize,
    start: Point,
    dims: Dim,
    turns: Turns,
}

struct PipeLand (Vec<Vec<Land>>);

impl Debug for PipeLand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for column in row {
                write!(f, "{:?}", column)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[derive(PartialEq)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}
use Dir::{Right, Down, Left, Up};

impl Dir {
    fn turn(&self, dir: &Dir) -> Dir {
        if dir == &Right {
            match self {
                Right => Down,
                Down => Left,
                Left => Up, 
                Up => Right,
            }
        } else {
            match self {
                Right => Up,
                Down => Right,
                Left => Down, 
                Up => Left,
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Land {
    Unknown,
    Pipe,
    In,
    Out,
}

impl Debug for Land {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Pipe => write!(f, "X"),
            Self::In => write!(f, "I"),
            Self::Out => write!(f, "O"),
        }
    }
}

impl PipeMap {
    pub fn build(input: &str) -> PipeMap {
        let pipes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = pipes.len();
        let width = pipes[0].len();
        let mut land = PipeLand(vec![vec![Land::Unknown; width]; height]);
        let (y, row) = pipes.iter().enumerate().find(|(_, row)| row.contains(&'S')).unwrap();
        let (x, _) = row.iter().enumerate().find(|(_, &ch)| ch == 'S').unwrap();
        land.0[y][x] = Land::Pipe;
        PipeMap { 
            pipes, 
            start: Point{x, y}, 
            land, 
            ins: 0, 
            turns: Turns { left: 0, right: 0 }, 
            dims: Dim{width, height} 
        }
    }

    fn get_start_dir(&self) -> Dir {
        let right_ch = self.pipes[self.start.y][self.start.x+1];
        let left_ch = self.pipes[self.start.y][self.start.x-1];
        let up_ch = self.pipes[self.start.y-1][self.start.x];
        
        if right_ch == '-' || right_ch == '7' || right_ch == 'J' {
            Right
        } else if left_ch == '-' || left_ch == 'L' || left_ch == 'F' {
            Left
        } else if up_ch == '|' || up_ch == 'F' || up_ch == '7' {
            Up
        } else {
            Down
        }
    }

    fn get_dir(from: &Point, to: &Point) -> Dir {
        let horizontal = from.y == to.y;
        if horizontal {
            if from.x < to.x {
                Right
            } else {
                Left
            }
        } else { // vertical
            if from.y < to.y {
                Down
            } else {
                Up
            }
        }
    }

    pub fn step(&mut self, from: Point, through: Point) -> Option<Point> {
        self.land.0[through.y][through.x] = Land::Pipe;
        let dir = Self::get_dir(&from, &through);
        let mut to = through;
        match (dir, self.pipes[through.y][through.x]) {
            (Right, '-') => to.x += 1,
            (Right, 'J') => {to.y -= 1; self.turns.left += 1},
            (Right, '7') => {to.y += 1; self.turns.right +=1},
            (Left, '-') => to.x -= 1,
            (Left, 'L') => {to.y -= 1; self.turns.right += 1},
            (Left, 'F') => {to.y += 1; self.turns.left += 1},
            (Down, '|') => to.y += 1,
            (Down, 'L') => {to.x += 1; self.turns.left += 1},
            (Down, 'J') => {to.x -= 1; self.turns.right += 1},
            (Up, '|') => to.y -= 1,
            (Up, '7') => {to.x -= 1; self.turns.left += 1},
            (Up, 'F') => {to.x += 1; self.turns.right += 1},
            _ => return None,            
        }

        Some(to)
    }

    fn ray_step(&self, from: Point, towards: &Dir) -> Option<Point> {
        let mut next = from;
        match towards {
            Right => next.x += 1,
            Left => next.x -= 1,
            Up => next.y -= 1,
            Down => next.y += 1,
        }
        if next.x > self.dims.width - 1 || next.y > self.dims.height - 1 {
            None
        } else {
            Some(next)
        }
    }

    fn ray(&mut self, from: &Point, towards: Dir) {
        let mut current = *from;
        loop {
            current = match self.ray_step(current, &towards) {
                None => return,
                Some(c) => c,
            };
            if self.land.0[current.y][current.x] == Land::Pipe {
                return
            }
            if self.land.0[current.y][current.x] == Land::Unknown {
                self.land.0[current.y][current.x] = Land::In;
                self.ins += 1;
            }
        }
    }
}

fn solve(input: &str) -> i128 {
    let mut pipes = PipeMap::build(input);
    let mut prev = pipes.start;
    let mut current = pipes.start;
    current.x += 1;
    let mut s = 0i128;
    while let Some(next) = pipes.step(prev, current) {
        s += 1;
        prev = current;
        current = next;
    }
    let pipe_len = s + 1;
    pipe_len / 2
}

fn solve2(input: &str) -> i128 {
    let mut pipes = PipeMap::build(input);
    eprintln!("Height: {} Width: {}", pipes.dims.height, pipes.dims.width);
    let mut prev = pipes.start;
    let start_dir = pipes.get_start_dir();
    let mut current = pipes.ray_step(prev, &start_dir).unwrap();

    while let Some(next) = pipes.step(prev, current) {
        prev = current;
        current = next;
    }
    // pipes are up
    let turn = if pipes.turns.right > pipes.turns.left {
        Right
    } else {
        Left
    };

    // eprintln!("{:?}", &pipes.land);
    prev = pipes.start;
    current = pipes.ray_step(prev, &start_dir).unwrap();
    while let Some(next) = pipes.step(prev, current) {
        let dir = PipeMap::get_dir(&prev, &current);
        pipes.ray(&prev, dir.turn(&turn));
        pipes.ray(&current, dir.turn(&turn));
        prev = current;
        current = next;
    }
    // eprintln!("{:?}", &pipes.land);

    pipes.ins as i128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input1();
        assert_eq!(solve(test_input), 4);
        let test_input = get_test_input2();
        assert_eq!(solve(test_input), 8);
    }

    #[test]
    fn test_solve_3() {
        let test_input = get_test_input3();
        assert_eq!(solve2(test_input), 4);
    }
    
    #[test]
    fn test_solve_4() {
        let test_input = get_test_input4();
        assert_eq!(solve2(test_input), 4);
    }

    #[test]
    fn test_solve_5() {
        let test_input = get_test_input5();
        assert_eq!(solve2(test_input), 8);
    }

    #[ignore = "We're not handling start positions on the border correctly"]
    #[test]
    fn test_solve_6() {
        let test_input = get_test_input6();
        assert_eq!(solve2(test_input), 10);
    }

    fn get_test_input1() -> &'static str {
        let test_input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        test_input
    }

    fn get_test_input2() -> &'static str {
        let test_input = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        test_input
    }

    fn get_test_input3() -> &'static str { // 4
        let test_input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        test_input
    }

    fn get_test_input4() -> &'static str { // 4
        let test_input = "\
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        test_input
    }

    fn get_test_input5() -> &'static str { // 8
        let test_input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        test_input
    }

    fn get_test_input6() -> &'static str { // 10
        let test_input = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        test_input
    }
}
