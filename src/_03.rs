use std::ops::Not;

use itertools::Itertools;

#[test]
fn part_one() {
    let mut answer =
        include_str!("../03.input")
            .trim()
            .lines()
            .collect_vec();
    let len = answer[0].len();
    let fake_line = ".".repeat(len);
    answer.insert(0, &fake_line);
    answer.push(&fake_line);
    let answer =
        answer.windows(3)
            .map(count_details)
            .sum::<i128>();
    println!("Day 3: Gear Ratios answer is {}", answer)
}

#[test] // todo
fn part_two() {
    let mut answer =
        include_str!("../03.input")
            .trim()
            .lines()
            .collect_vec();
    let len = answer[0].len();
    let fake_line = ".".repeat(len);
    answer.insert(0, &fake_line);
    answer.push(&fake_line);
    let answer =
        answer.windows(3)
            .map(count_gears_ratio)
            .sum::<i128>();
    println!("Part Two answer is {}", answer)
}

fn count_gears_ratio(triple_windows: &[&str]) -> i128 {
    let top = triple_windows[0];
    let current = triple_windows[1];
    let bottom = triple_windows[2];
    let gears_indices = find_gears_indices(current);
    let mut ratio_sum = 0;
    for i in gears_indices {
        let top_number = find_number_for_gear(top, i);
        let bottom_number = find_number_for_gear(bottom, i);
        if let (Some(top_number), Some(bottom_number)) = (top_number, bottom_number) {
            let ratio = top_number * bottom_number;
            ratio_sum += ratio;
        }
    }
    ratio_sum
}

fn find_number_for_gear(s: &str, gear_idx: usize) -> Option<i128> {
    let number_indices = find_possible_number_indices(gear_idx, s.len() - 1);
    for i in number_indices {
        let parsed_number = try_parse_number(s, i);
        if parsed_number.is_some() {
            return parsed_number;
        }
    }
    None
}

fn try_parse_number(s: &str, any_idx: usize) -> Option<i128> {
    let char = char_at(s, any_idx);
    if char.is_ascii_digit() {
        let number =
            format!("{}{}{}",
                    digits_before(s, any_idx),
                    char,
                    digits_after(s, any_idx)
            );
        return number.parse::<i128>().ok();
    }
    None
}

fn digits_before(s: &str, any_idx: usize) -> String {
    if any_idx == 0 {
        return String::from("");
    }
    let s = s[0..any_idx].chars().rev().collect::<String>();
    let mut stop = s.len();
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit().not() {
            stop = i;
            break;
        }
    }
    String::from(&s[0..stop])
}

fn digits_after(s: &str, any_idx: usize) -> &str {
    if any_idx == s.len() - 1 {
        return "";
    }
    let s = &s[any_idx + 1..];
    let mut stop = s.len();
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit().not() {
            stop = i;
            break;
        }
    }
    &s[0..stop]
}

fn find_possible_number_indices(gear_idx: usize, limit: usize) -> Vec<usize> {
    let mut result = Vec::new();
    if gear_idx > 0 {
        result.push(gear_idx - 1)
    }
    result.push(gear_idx);
    if gear_idx < limit {
        result.push(gear_idx + 1)
    }
    result
}

fn find_gears_indices(s: &str) -> Vec<usize> {
    s.char_indices()
        .filter(|(i, c)| *c == '*')
        .map(|(i, c)| i)
        .collect_vec()
}

fn count_details(triple_windows: &[&str]) -> i128 {
    let top = triple_windows[0];
    let curr = triple_windows[1];
    let bottom = triple_windows[2];
    let mut sum = 0;
    let mut from = None;
    let mut to = None;
    for (idx, char) in curr.char_indices() {
        if char.is_ascii_digit() {
            if from.is_none() {
                from = Some(idx)
            }
            to = Some(idx)
        } else {
            sum = try_increment(sum, from, to, top, curr, bottom);
            from = None;
            to = None;
        }
    }
    try_increment(sum, from, to, top, curr, bottom)
}

fn try_increment(sum: i128, from: Option<usize>, to: Option<usize>, top: &str, curr: &str, bottom: &str) -> i128 {
    if let Some(detail) = parse_valid_detail(from, to, top, curr, bottom) {
        sum + detail
    } else {
        sum
    }
}

fn parse_valid_detail(from: Option<usize>, to: Option<usize>, top: &str, curr: &str, bottom: &str) -> Option<i128> {
    if let (Some(from), Some(to)) = (from, to) {
        if is_valid_detail(from, to, top, curr, bottom) {
            return Some(parse_number(from, to, curr));
        }
    }
    None
}

fn is_valid_detail(from: usize, to: usize, top: &str, curr: &str, bottom: &str) -> bool {
    if from > 0 {
        let before_from = from - 1;
        if is_detail(char_at(top, before_from)) {
            return true;
        }
        if is_detail(char_at(curr, before_from)) {
            return true;
        }
        if is_detail(char_at(bottom, before_from)) {
            return true;
        }
    }
    let after_to = to + 1;
    if after_to < curr.len() {
        if is_detail(char_at(top, after_to)) {
            return true;
        }
        if is_detail(char_at(curr, after_to)) {
            return true;
        }
        if is_detail(char_at(bottom, after_to)) {
            return true;
        }
    }
    let top = &top[from..after_to];
    let bottom = &bottom[from..after_to];
    let border = format!("{}{}", top, bottom);
    for char in border.chars() {
        if is_detail(char) {
            return true;
        }
    }
    false
}

fn char_at(s: &str, i: usize) -> char {
    s.chars().nth(i).unwrap()
}

fn is_detail(char: char) -> bool {
    return char.is_ascii_digit().not() && char != '.';
}

fn parse_number(from: usize, to: usize, str: &str) -> i128 {
    str[from..(to + 1)].parse().unwrap()
}