use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x,
            y,
        }
    }
}

struct Grid {
    state: [[bool; 1000]; 1000],
    brightness: [[i8; 1000]; 1000],
}

impl Grid {
    fn new() -> Grid {
        Grid {
            state: [[false; 1000]; 1000],
            brightness: [[0; 1000]; 1000],
        }
    }

    fn turn_on(&mut self, start: &Point, end: &Point) {
        for x in start.x..end.x + 1 {
            for y in start.y..end.y + 1 {
                self.state[x][y] = true;
                self.brightness[x][y] += 1;
            }
        }
    }

    fn turn_off(&mut self, start: &Point, end: &Point) {
        for x in start.x..end.x + 1 {
            for y in start.y..end.y + 1 {
                self.state[x][y] = false;
                if self.brightness[x][y] > 0 { self.brightness[x][y] -= 1 }
            }
        }
    }

    fn toggle(&mut self, start: &Point, end: &Point) {
        for x in start.x..end.x + 1 {
            for y in start.y..end.y + 1 {
                self.state[x][y] = !self.state[x][y];
                self.brightness[x][y] += 2
            }
        }
    }

    fn count_lights(&self) -> i32 {
        let mut sum = 0;
        for x in 0..self.state.len() {
            for y in 0..self.state[x].len() {
                if self.state[x][y] {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn total_brightness(&self) -> i64 {
        let mut sum:i64 = 0;
        for x in 0..self.brightness.len() {
            for y in 0..self.brightness[x].len() {
                sum += self.brightness[x][y] as i64
            }
        }
        sum
    }
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
    Unknown,
}

pub fn ex6() {
    let mut grid = Grid::new();
    let f = File::open("resources/2015/ex6_in").expect("unable to read file");
    let f = BufReader::new(f);
    for line in f.lines() {
        if let Ok(txt) = line {
            let tokens: Vec<&str> = txt.split_whitespace()
                .filter(|x| x != &"turn" && x != &"through")
                .collect();
            let mut tokens_iter = tokens.iter();
            let action = parse_action(tokens_iter.next().unwrap());
            let start_point = parse_point(tokens_iter.next().unwrap());
            let end_point = parse_point(tokens_iter.next().unwrap());

            match action {
                Action::TurnOn => grid.turn_on(&start_point, &end_point),
                Action::TurnOff => grid.turn_off(&start_point, &end_point),
                Action::Toggle => grid.toggle(&start_point, &end_point),
                Action::Unknown => println!("Unknown action!")
            }
        }
    }
    eprintln!("grid.count_lights() = {:?}", grid.count_lights());
    eprintln!("grid.total_brightness() = {:?}", grid.total_brightness());
}

fn parse_action(token: &str) -> Action {
    match token {
        "on" => Action::TurnOn,
        "off" => Action::TurnOff,
        "toggle" => Action::Toggle,
        _ => Action::Unknown
    }
}

fn parse_point(token: &str) -> Point {
    let mut iter = token.split(",");
    let x = parse_coord(iter.next());
    let y = parse_coord(iter.next());
    Point::new(x, y)
}

fn parse_coord(txt: Option<&str>) -> usize {
    txt.unwrap()
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn turn_on_all() {
        let mut grid = Grid::new();
        grid.turn_on(&Point::new(0, 0), &Point::new(999, 999));
        assert_eq!(grid.count_lights(), 1000000)
    }

    #[test]
    fn turn_on_row() {
        let mut grid = Grid::new();
        grid.turn_on(&Point::new(0, 0), &Point::new(999, 0));
        assert_eq!(grid.count_lights(), 1000)
    }

    #[test]
    fn turn_on_central_four() {
        let mut grid = Grid::new();
        grid.turn_on(&Point::new(499, 499), &Point::new(500, 500));
        assert_eq!(grid.count_lights(), 4)
    }
}