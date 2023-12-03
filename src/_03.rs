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

#[test]
fn part_two() {

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