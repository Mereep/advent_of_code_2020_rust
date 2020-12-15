use crate::files::str_to_lines;
use regex::Regex;

pub fn task1 () {
    let data = get_input_data();
    let lines = str_to_lines(data);

    // iterate over all characters of the password
    let mut n_valid_passwords = 0;

    for line in lines {
        let parsed_line = parse_line(&line);
        let min = parsed_line.0;
        let max = parsed_line.1;

        // this basically extracts the first character of a &str
        // (which only has exactly one anyways)
        let needle = parsed_line.2.chars().next().unwrap();
        let password = parsed_line.3;

        // iterate over all characters of the password
        let mut n_found = 0;
        for char in password.chars() {
            // and cound how often we meet [needle] in the [password]
            if char == needle {
                n_found+=1;
            }

            if n_found > max {
                break;
            }
        }

        if n_found >= min && n_found <= max {
            n_valid_passwords += 1;
        }
    }

    println!("Valid passwords (Task 1): {}", n_valid_passwords);
}

pub fn task2 () {
    let data = get_input_data();
    let lines = str_to_lines(data);

    // iterate over all characters of the password
    let mut n_valid_passwords = 0;

    for line in lines {
        let parsed_line = parse_line(&line);
        let first_pos = parsed_line.0;
        let second_pos = parsed_line.1;

        // this basically extracts the first character of a &str
        // (which only has exactly one anyways)
        let needle = parsed_line.2.chars().next().unwrap();
        let password = parsed_line.3;

        // Now check the first and second position
        // if exactly one (not none and also not both -> xor(^)) times the [needle] appears
        if (password.chars().nth(first_pos as usize - 1).unwrap() == needle) ^ (password.chars().nth(second_pos as usize - 1).unwrap() == needle) {
            n_valid_passwords += 1;
        }

    }

    println!("Valid passwords (Task 2): {}", n_valid_passwords);
}


/// Parses a line like `2-9 c: ccccccccc´
/// and returns (2, 9, 'c', 'ccccccccc')
fn parse_line(line: &str) -> (u8, u8, &str, &str) {
    // those (?P<name>) things generate a named match that we can extract later on
    let re = Regex::new(r"^(?P<from>[0-9]+)-(?P<to>[0-9]+) (?P<needle>.): (?P<password>.+)$").expect("Regex not valid");
    let matches = re.captures(line).expect(&format!("Regex did not match for line `{}´", line));

    return (matches["from"].parse::<u8>().unwrap(),
            matches["to"].parse::<u8>().unwrap(),
            matches.name("needle").unwrap().as_str(),
            matches.name("password").unwrap().as_str()
    )
}


/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}
