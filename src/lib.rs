#[cfg(test)]
mod aof_2023 {
    use std::collections::HashMap;

    #[test]
    fn aof_1_1() {
        let answer =
            include_str!("../input.1")
                .split('\n')
                .map(extract_number)
                .sum::<i32>();
        println!("aof 2023 1.1 answer is {}", answer)
    }

    #[test]
    fn aof_1_2() {
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
                .split('\n')
                .map(|l| extract_ext_number(&written_digit_to_digit, l))
                .sum::<i32>();
        println!("aof 2023 1.2 answer is {}", answer)
    }

    fn extract_ext_number(digits: &HashMap<&str, char>, line: &str) -> i32 {
        let first_digit = get_first_ext_digit(digits, line);
        let last_digit = get_last_ext_digit(digits, line);
        join_digits_to_two_digit_number(first_digit, last_digit)
    }

    fn get_first_ext_digit(digits: &HashMap<&str, char>, line: &str) -> char {
        for (&key, value) in digits {
            if line.starts_with(key) {
                return *value;
            }
        }
        get_first_ext_digit(digits, &line[1..])
    }

    fn get_last_ext_digit(digits: &HashMap<&str, char>, line: &str) -> char {
        for (&key, value) in digits {
            if line.ends_with(key) {
                return *value;
            }
        }
        get_last_ext_digit(digits, &line[..(line.len() - 1)])
    }

    fn extract_number(line: &str) -> i32 {
        let first_digit = get_first_digit(line);
        let last_digit = get_last_digit(line);
        join_digits_to_two_digit_number(first_digit, last_digit)
    }

    fn join_digits_to_two_digit_number(first_digit: char, last_digit: char) -> i32 {
        let two_digit_number = format!("{}{}", first_digit, last_digit);
        two_digit_number.parse().unwrap()
    }

    fn get_first_digit(line: &str) -> char {
        line.chars().find(char::is_ascii_digit).unwrap()
    }

    fn get_last_digit(line: &str) -> char {
        line.chars().rev().find(char::is_ascii_digit).unwrap()
    }
}
