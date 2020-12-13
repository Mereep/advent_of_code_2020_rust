use std::fs::File;
use std::io::{BufRead};

pub fn read_file_to_lines(filename: &str) -> Vec<String> {
    let maybe_file = File::open(filename);

    return match maybe_file {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let lines: Vec<String> = reader.lines().into_iter()
                .map(|line| line.unwrap())
                .collect();
            lines
        },
        Err(err) => {
            panic!("Couldn't read input file due to `{}´", err)
        }
    }
}

/// Splits a str on newlines and returns
/// every line as a vector member
pub fn str_to_lines(lines: &str) -> Vec<String> {
    lines.lines().into_iter().map(|line| String::from(line)).collect()
}
