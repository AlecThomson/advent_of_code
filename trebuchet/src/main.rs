use std::fs::read_to_string;
use std::{env, process, io};
use std::io::ErrorKind;

// From Rust by example
// Handle the error though
fn read_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let reader = read_to_string(filename);
    let mut reader = match reader {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    Ok(reader.lines().map(String::from).collect())
}

fn decode(code: &str) -> i32 {
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
    let mut running_total = 0;
    for code in codes {
        let value = decode(&code);
        running_total += value;
    }
    return running_total
}

fn parse_args() -> String {
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
    // Parse CLI
    let file_to_read = parse_args();
    let codes = read_lines(&file_to_read);
    let calibration = calibrate(codes);
    println!("Calibration total is {}", calibration);


}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn test_decode() {
        assert_eq!(decode("1abc2"), 12);
        assert_eq!(decode("pqr3stu8vwx"), 38);
        assert_eq!(decode("a1b2c3d4e5f"), 15);
        assert_eq!(decode("treb7uchet"), 77);
    }
}
