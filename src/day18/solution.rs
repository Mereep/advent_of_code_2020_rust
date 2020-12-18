use crate::files::str_to_lines;
use regex::Regex;

pub fn task1(with_operator_precedence: bool) {
    let data = get_input_data();
    let lines = str_to_lines(data);

    let results: Vec<i64> = lines
        .into_iter()
        .map(|line| compute_formula(&line, with_operator_precedence))
        .collect();

    println!("Results: {:?} (with precendece={:?})", results, with_operator_precedence);
    println!("Sum of results {} (with precendece={:?})", results.into_iter().sum::<i64>(), with_operator_precedence);
}


pub fn task2() {
    task1(true);
}

/// Inicates what we are doing right now
enum State {
    ReadsLeftOperand,
    ReadsRightOperand,
    ReadsOperator
}

/// computes the equation
///
/// if [with_operator_precedence] is [true] we will execute + before *
/// otherwise all operators are treated equal
pub fn compute_formula(formula: &str, with_operator_precedence: bool) -> i64 {

    // Read position
    let mut idx = 0 as usize;

    // Context of our `machine´
    let mut state = State::ReadsLeftOperand;

    // just reads an integer
    let regex_integer = Regex::new(r"\d+").expect("Integer regex not compileable");
    let mut curr_left_value: i64 = 0;
    let mut curr_operator: char = ' ';
    loop {
        // Read the character and just break if we're done
        let maybe_curr_char = formula.chars().nth(idx);
        if maybe_curr_char.is_none() {break};
        let curr_char = maybe_curr_char.unwrap();

        // Stores a value which was read in this step
        let mut read_value: i64= 0;

        // println!("Current idx {}: {}", idx, curr_char);
        match curr_char {
            '(' => {
                let inner_part_end_idx = read_until_matching_bracket(formula, idx + 1);
                let inner_result = compute_formula(&formula[idx + 1..inner_part_end_idx + 1], with_operator_precedence);
                idx = inner_part_end_idx + 1;
                read_value = inner_result;
            },
            '0' ..= '9' => {
                let number_match = regex_integer.captures(&formula[idx..])
                    .expect(&format!("Couldn't parse number beginning from index {}", idx));

                read_value = number_match[0].parse::<i64>()
                    .expect(&format!("Couldn't parse number {} as integer", &number_match[0]));
                // println!("Read value parse {}", read_value);
                let match_size = number_match[0].len();
                idx += match_size;
            },
            ' ' => { // White spaces are just eaten
                idx += 1;
                continue
            },
            '+'  => {
                curr_operator = curr_char;
            },
            '*' => {
                if with_operator_precedence {
                    let right_part = compute_formula(&formula[idx + 1..], with_operator_precedence);
                    return curr_left_value * right_part;
                } else {
                    curr_operator = curr_char.clone();
                }
            },
            _ => {
                panic!(format!("Read unknown char `{}´ at index {}", curr_char, idx));
            }
        }


        match state {
            State::ReadsLeftOperand => {
                // println!("read value {}", read_value);
                curr_left_value = read_value;
                state = State::ReadsOperator;
            },

            State::ReadsRightOperand => {
                curr_left_value = match curr_operator {
                    '+' => curr_left_value + read_value,
                    '-' => curr_left_value - read_value,
                    '*' => curr_left_value * read_value,
                    _ => panic!("Operator unknown")
                };
                // println!("Left value {} right value {}", curr_left_value, read_value);
                state = State::ReadsOperator;
            },
            State::ReadsOperator => {
                state = State::ReadsRightOperand;
            }
        }
        idx+=1;
    }

    return curr_left_value;
}

/// Reads until it finds a closing bracket `)´
///
/// returns position BEFORE closing bracket
fn read_until_matching_bracket(formula_part: &str, start_idx: usize) -> usize {
    let mut n_opening_brackets = 1 as u64;
    for (offset, char) in formula_part.chars().skip(start_idx).into_iter().enumerate() {
        if char == '(' {
            n_opening_brackets += 1;
        } else if char == ')' {
            n_opening_brackets -= 1;
        }

        if n_opening_brackets == 0 {
            return start_idx + offset - 1;
        }
    }

    panic!(format!("Didn't find matching closing bracket beginning from index {}", start_idx))
}


/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}