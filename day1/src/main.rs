use std::collections::HashMap;

fn main() {
    let input = include_str!("/Users/brendan/Desktop/adventofcode2023/input.txt");
    let mut sum = 0;
    for line in input.split("\n") {
        let mut digits = line.chars().filter(|x| x.is_digit(10));
        let first = digits.next().unwrap();
        let last = match digits.last() {
            Some(digit) => format!("{}", digit),
            None => format!("{}", first),
        };
        let val = format!("{}{}", first, last).parse::<i32>().unwrap();
        sum += val;
    }
    println!("{}", sum);

    let word_numbers = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut sum2 = 0;
    for line in input.split("\n") {
        let mut first = String::from("");
        let mut last = String::from("");

        for i in 0..line.len() {
            if line[i..i + 1].chars().next().unwrap().is_digit(10) {
                first = format!("{}", line[i..i + 1].chars().next().unwrap());
                break;
            }
            for (substr, digit) in word_numbers.iter() {
                if line[i..].starts_with(substr) {
                    first = format!("{}", digit);
                    break;
                }
            }
            if first != "" {
                break;
            }
        }

        for i in (0..line.len()).rev() {
            if line[i..i + 1].chars().next().unwrap().is_digit(10) {
                last = format!("{}", line[i..i + 1].chars().next().unwrap());
                break;
            }
            for (substr, digit) in word_numbers.iter() {
                if line[i..].starts_with(substr) {
                    last = format!("{}", digit);
                    break;
                }
            }
            if last != "" {
                break;
            }
        }

        let val = format!("{}{}", first, last).parse::<i32>().unwrap();
        sum2 += val;
    }
    println!("{}", sum2);
}
