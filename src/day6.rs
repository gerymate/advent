use std::fs::read_to_string;

pub fn solve_day_6() -> i32 {
    let input = read_to_string("../input6.txt").unwrap();
    solve2(&input) as i32
}

#[derive(Copy, Clone)]
struct Race {
    time: i64,
    record: i64,
}

impl Race {
    fn new((time, record):(i64, i64)) -> Self {
        Self { time, record }
    }

    fn record_passers(self) -> i64 {
        let time = self.time as f64;
        let record = self.record as f64;
        let first_passer = (-time - (time * time - 4.0 * record).sqrt()) / 2.0; 
        let last_passer = (-time + (time * time - 4.0 * record).sqrt()) / 2.0;
        println!("first f: {}, last f: {}", first_passer, last_passer);
        let mut first_passer_i = first_passer.ceil() as i64;
        if first_passer == first_passer.ceil() {
            first_passer_i += 1;
        }
        let mut last_passer_i = last_passer.floor() as i64;
        if last_passer == last_passer.floor() {
            last_passer_i -= 1;
        }
        println!("first i: {}, last i: {}", first_passer_i, last_passer_i);
        last_passer_i - first_passer_i + 1
    }
}

fn solve(input: &str) -> i64 {
    // parse input
    let mut lines = input.lines();
    let timeline = lines.next().unwrap().split_whitespace().skip(1)
        .map(|c| c.parse::<i64>().unwrap());
    let distances = lines.next().unwrap().split_whitespace().skip(1)
        .map(|c| c.parse::<i64>().unwrap());
    let racebuilder = timeline.zip(distances);
    let races = racebuilder.map(Race::new).collect::<Vec<_>>();

    // we seek all integer values when tb * (time - tb) > record
    /* ie. tb is between solutions to 
    tb * time - tb^2 = record
    -tb^2 + time*tb - record = 0
    tb12 = (-time +/- sqrt(time^2 - 4 * record) ) 
            ------------------------------------ 
                        -2
    ceil ((-time - sqrt(time^2 - 4*record)) / 2 ) 
    
     */

    races.iter().map(|r| r.record_passers())
        .inspect(|c| println!("c: {}", c))
        .product()
}

fn solve2(input: &str) -> i64 {
    let mut lines = input.lines();
    let time = lines.next().unwrap().split_whitespace().skip(1)
        .fold("".to_owned(), |a, f| a.to_owned() + f)
        .parse::<i64>().unwrap();
    let distance = lines.next().unwrap().split_whitespace().skip(1)
        .fold("".to_owned(), |a, f| a.to_owned() + f)
        .parse::<i64>().unwrap();
    let race = Race::new((time, distance));
    race.record_passers()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = get_test_input();
        assert_eq!(solve(test_input), 288);
    }

    #[test]
    fn test_solve2() {
        let test_input = get_test_input();
        assert_eq!(solve2(test_input), 71503);
    }

    fn get_test_input() -> &'static str {
        let test_input = "\
Time:      7  15   30
Distance:  9  40  200";
        test_input
    }
}
