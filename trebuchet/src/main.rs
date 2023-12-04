use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::exit;
use std::{env, io};
use std::time::Instant;
use rayon::prelude::*;
use indexmap::IndexMap;

// From Rust by example
// Handle the error though
fn read_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let contents_result = read_to_string(filename);
    let contents = match contents_result {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    Ok(contents.lines().map(String::from).collect())
}

// Find spelled out numbers
fn text_to_number(code: String) -> String {
    let numbers = IndexMap::from(
        [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4), 
            ("five", 5),
            ("six", 6), 
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
    );
    let numbers_inverted: IndexMap<i32, &str> = numbers.iter()
        .map(|(k, v)| (v.clone(), k.clone())).collect();
    let mut number_check =  IndexMap::from(
        [
            ("one", false),
            ("two", false),
            ("three", false),
            ("four", false), 
            ("five", false),
            ("six", false), 
            ("seven", false),
            ("eight", false),
            ("nine", false),
        ]
    );
    for number in numbers.keys() {
        number_check.insert(number, code.contains(number));
    }
    if ! number_check.values().any(|&x| x) {
        return code
    }
    // let values =  number_check.values();
    // println!("{values:?}");
    let mut new_code = code.clone();
    println!("number_check={number_check:?}");
    for (i, &check) in number_check.values().enumerate() {
        let num = i + 1;
        if check {
            new_code = new_code.replace(
                numbers_inverted[i],
                num.to_string().as_str(),
            )
        }
    }
    // for (key, value )in numbers {
    //     if new_code.contains(key) {
    //         println!("{key}/{value} is in {new_code}");
    //         new_code = new_code.replace(key, value.to_string().as_str());
    //         println!("new_code={new_code}");
    //     }
    // }
    new_code
}

fn decode(code: &str) -> i32 {
    // Decode the elvish codes into numbers
    // String of letters and numbers
    // Return left-most and right-most numbers in code
    let mut number_string: String = "".to_owned();
    // Loop over 'code' to get left-most number
    for c in code.chars() {
        if c.is_numeric() {
            number_string.push(c);
            break
        }
    }
    // Loop over reversed 'code' to get right-most
    for c in code.chars().rev() {
        if c.is_numeric() {
            number_string.push(c);
            break
        }
    }
    let number_int: i32 = number_string.parse().unwrap();
    number_int
}

fn calibrate(codes: Vec<String>) -> i32 {
    // Find the calibration code for the Elves
    return codes
        .par_iter()
        .map(|c| decode(c))
        .sum();
}

fn parse_args() -> String {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();
    let file_to_read = 
        if args.len() > 1 {
            args[1].clone()
    } else {
        String::from("tests/example.txt")
    };
    println!("Reading {}", file_to_read);
    file_to_read
}

fn main() {
    let now = Instant::now();

    let file_to_read = parse_args();
    let codes_result = read_lines(&file_to_read);
    let codes = match codes_result {
        Ok(c) => c,
        Err(e) => {
            println!("Error reading {}: {}", file_to_read, e);
            exit(1)
        }
    };
    let calibration = calibrate(codes);
    println!("Calibration total is {}", calibration);
    println!("Execution took {}µs", now.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use crate::{decode, calibrate, text_to_number};

    #[test]
    fn test_decode() {
        assert_eq!(decode("1abc2"), 12);
        assert_eq!(decode("pqr3stu8vwx"), 38);
        assert_eq!(decode("a1b2c3d4e5f"), 15);
        assert_eq!(decode("treb7uchet"), 77);
    }

    #[test]
    fn test_calibrate() {
        assert_eq!(
            calibrate(
                vec![
                    "1abc2".to_string(), 
                    "pqr3stu8vwx".to_string(), 
                    "a1b2c3d4e5f".to_string(), 
                    "treb7uchet".to_string(),
                ]
            ),
            142,
        )
    }

    #[test]
    fn test_text_to_number() {
        assert_eq!(
            text_to_number("two1nine".to_string()), 
            "219".to_string()
        );
        assert_eq!(
            text_to_number("eightwothree".to_string()), 
            "8wo3".to_string()
        );
        assert_eq!(
            text_to_number("abcone2threexyz".to_string()), 
            "abc123xyz".to_string()
        );
        assert_eq!(
            text_to_number("xtwone3four".to_string()), 
            "x2ne34".to_string()
        );
        assert_eq!(
            text_to_number("4nineeightseven2".to_string()), 
            "49872".to_string()
        );
        assert_eq!(
            text_to_number("zoneight234".to_string()), 
            "z18234".to_string()
        );
        assert_eq!(
            text_to_number("7pqrstsixteen".to_string()), 
            "7pqrst6teen".to_string()
        );
    }
}
