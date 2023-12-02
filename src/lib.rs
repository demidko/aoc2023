use std::collections::HashMap;

#[test]
pub fn aof_1_1() {
    let answer =
        include_str!("../input.1")
            .lines()
            .map(extract_number)
            .sum::<i32>();
    println!("aof 2023 1.1 answer is {}", answer)
}

#[test]
pub fn aof_1_2() {
    let written_digit_to_digit =
        HashMap::from([
            ("1", '1'),
            ("one", '1'),
            ("2", '2'),
            ("two", '2'),
            ("3", '3'),
            ("three", '3'),
            ("4", '4'),
            ("four", '4'),
            ("5", '5'),
            ("five", '5'),
            ("6", '6'),
            ("six", '6'),
            ("7", '7'),
            ("seven", '7'),
            ("8", '8'),
            ("eight", '8'),
            ("9", '9'),
            ("nine", '9'),
        ]);
    let answer =
        include_str!("../input.1")
            .lines()
            .map(|l| extract_ext_number(&written_digit_to_digit, l))
            .sum::<i32>();
    println!("aof 2023 1.2 answer is {}", answer)
}

#[test]
pub fn aof_2_1() {
    let answer = include_str!("../input.2").lines().filter_map(parse_game).sum::<i128>();
    println!("aof 2023 2.1 answer is {}", answer)
}

#[test]
pub fn aof_2_2() {
    let answer = include_str!("../input.2").lines().map(parse_game_power).sum::<i128>();
    println!("aof 2023 2.2 answer is {}", answer)
}

pub fn parse_game_power(game: &str) -> i128 {
    let mut game = game.split(": ");
    game.next();
    let rgb_sets =
        game.next().unwrap()
            .split("; ")
            .map(Rgb::from_color_set)
            .collect::<Vec<Rgb>>();
    let rgb = Rgb::merge(&rgb_sets);
    (rgb.r * rgb.g * rgb.b) as i128
}

pub fn parse_game(src: &str) -> Option<i128> {
    let mut game = src.split(": ");
    let mut id = game.next().unwrap().split_whitespace();
    let color_sets = game.next().unwrap().split("; ");
    for color_set in color_sets {
        let mut rgb = Rgb::new();
        rgb.minus_colors_set(color_set);
        if rgb.is_valid() {
            continue;
        }
        return None;
    }
    id.next();
    id.next().unwrap().parse::<i128>().ok()
}

#[derive(Debug)]
pub struct Rgb {
    r: i32,
    g: i32,
    b: i32,
}

impl Rgb {
    fn from_color_set(color_set: &str) -> Rgb {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for color_code in color_set.split(", ") {
            let mut color_code = color_code.split(" ");
            let counter = color_code.next().unwrap().parse::<i32>().unwrap();
            let color = color_code.next().unwrap();
            match color {
                "red" => r = counter,
                "green" => g = counter,
                "blue" => b = counter,
                _ => panic!("Unknown color {}", color)
            }
        }
        Self { r, g, b }
    }

    fn merge(vec: &Vec<Rgb>) -> Rgb {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for rgb in vec {
            if rgb.r > r {
                r = rgb.r
            }
            if rgb.b > b {
                b = rgb.b
            }
            if rgb.g > g {
                g = rgb.g
            }
        }
        Self { r, g, b }
    }

    fn new() -> Rgb {
        Self {
            r: 12,
            g: 13,
            b: 14,
        }
    }

    fn minus_color(&mut self, code: &str) {
        let mut code = code.split_whitespace();
        let number = code.next().unwrap().parse::<i32>().unwrap();
        let color = code.next().unwrap();
        match color {
            "green" => self.g -= number,
            "blue" => self.b -= number,
            "red" => self.r -= number,
            _ => panic!("Unknown color {}", color)
        }
    }

    fn minus_colors_set(&mut self, code: &str) {
        for color_code in code.split(", ") {
            self.minus_color(color_code);
        }
    }

    fn is_valid(&self) -> bool {
        self.r >= 0 && self.g >= 0 && self.b >= 0
    }
}


pub fn extract_ext_number(digits: &HashMap<&str, char>, line: &str) -> i32 {
    let first_digit = get_first_ext_digit(digits, line);
    let last_digit = get_last_ext_digit(digits, line);
    join_digits_to_two_digit_number(first_digit, last_digit)
}

pub fn get_first_ext_digit(digits: &HashMap<&str, char>, line: &str) -> char {
    for (&key, value) in digits {
        if line.starts_with(key) {
            return *value;
        }
    }
    get_first_ext_digit(digits, &line[1..])
}

pub fn get_last_ext_digit(digits: &HashMap<&str, char>, line: &str) -> char {
    for (&key, value) in digits {
        if line.ends_with(key) {
            return *value;
        }
    }
    get_last_ext_digit(digits, &line[..(line.len() - 1)])
}

pub fn extract_number(line: &str) -> i32 {
    let first_digit = get_first_digit(line);
    let last_digit = get_last_digit(line);
    join_digits_to_two_digit_number(first_digit, last_digit)
}

pub fn join_digits_to_two_digit_number(first_digit: char, last_digit: char) -> i32 {
    let two_digit_number = format!("{}{}", first_digit, last_digit);
    two_digit_number.parse().unwrap()
}

pub fn get_first_digit(line: &str) -> char {
    line.chars().find(char::is_ascii_digit).unwrap()
}

pub fn get_last_digit(line: &str) -> char {
    line.chars().rev().find(char::is_ascii_digit).unwrap()
}

