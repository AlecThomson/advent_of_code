use std::fs::read_to_string;
use std::process::exit;
use std::{env, io};
use std::time::Instant;
use rayon::prelude::*;

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
    return number_int
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
            String::from(args[1].clone())
    } else {
        String::from("tests/example.txt")
    };
    println!("Reading {}", file_to_read);
    return file_to_read
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
    use crate::{decode, calibrate};

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
}
