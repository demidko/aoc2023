#[test]
fn part_one() {
    let answer = include_str!("../02.input").trim().lines().filter_map(parse_game).sum::<i128>();
    println!("{}", answer)
}

#[test]
fn part_two() {
    let answer = include_str!("../02.input").trim().lines().map(parse_game_power).sum::<i128>();
    println!("{}", answer)
}

fn parse_game_power(game: &str) -> i128 {
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

fn parse_game(src: &str) -> Option<i128> {
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

struct Rgb {
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