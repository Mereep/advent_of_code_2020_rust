use std::collections::HashMap;
use crate::files::str_to_lines;
use regex::Regex;
use num::range;

pub fn task1() {
    let input = get_input_data();
    let lines = str_to_lines(input);

    // This map stores the memory of our machine
    let mut memory: HashMap<u64, u64> = HashMap::new();


    // Those bits have to be set to one
    let mut current_mask_1: u64 = 0;

    // Those bits have to be set to 0
    let mut current_mask_0: u64 = 0;

    // Execute program
    for (i, line) in lines.iter().enumerate() {
      if line.starts_with("mask") {
          let (current_mask_1, current_mask_0, _) = parse_line_mask(line.as_str());

      } else {
          let (index, value) = parse_line_memory(line.as_str());
          let effective_value = apply_masks_to_value(value,
                                                     &current_mask_1,
                                                     &current_mask_0);

          println!("Calucaltion (line: {}):\n  {:064b} ({})\n| {:064b}\n& {:064b}\n= {:064b} ({})\n", i, value, value, current_mask_1, !current_mask_0, effective_value, effective_value);

          memory.insert(index, effective_value);
      }
    }

    println!("Memory: {:?},\nSumme (Lösung): {}", memory, memory.values().sum::<u64>());
}

pub fn task2() {
    let input = get_input_data();
    let lines = str_to_lines(input);

    // This map stores the memory of our machine
    let mut memory: HashMap<u64, u64> = HashMap::new();

    // Those bits are forced to be set to one
    let mut current_floating_mask: u64 = 0;

    // Those will have to be BOTH, 1 and 0
    let mut current_set_to_one_mask: u64 = 0;

    for (i, line) in lines.iter().enumerate() {
        println!("Processing line {}: `{}´", i, line);
        if line.starts_with("mask") { // Parse a mask line

            let masks = parse_line_mask(line.as_str());
            current_set_to_one_mask = masks.0;
            current_floating_mask = masks.2;

        } else {
            let index_and_value = parse_line_memory(line.as_str());
            let mut memory_address = index_and_value.0;
            let value = index_and_value.1;

            // This time the memory address has to be switched
            memory_address |= current_set_to_one_mask;
            // put in at default memory address

            fill_memory(&mut memory,
                        &value,
                        0,
                        &current_floating_mask,
                        memory_address);

        }
    }
    println!("Sum of memory: {}", memory.values().sum::<u64>());
}

/// The idea here is that each true bit in the [floating_mask] will
/// cause one branch of processing (one where the corresponding bit in the memory address is forced
/// to one and one where it is forced to 0)
///
/// We enumerate over all possible cases of resulting memory addresses recursively beginning
/// from the least significant bit (the lowest); assumed memory layout is big endian
///
/// We will always pass the current address index [index] through all calls
fn fill_memory(memory: &mut HashMap<u64, u64>,
               value: &u64,
               index: u64,
               &floating_mask: &u64,
               current_memory_address: u64) {

    // if we are at the most significant bit, we just are done and can write to this memory
    // address
    if index == 36 {
        memory.insert(current_memory_address, *value);
        return;
    }


    // We check the right most bit of the floating mask if it is 1
    let is_floating_bit =
        ((floating_mask >> index) & (1 as u64)) == 1;

    // if its not some bit we can switch (non-floating) we just ignore it
    if is_floating_bit {
        for i in range(0,2) {   // Switch bit to 0 and 1, respectively
            let flipped_memory_address = match i {
                0 =>  current_memory_address & !((1 as u64) << index),
                1 =>  current_memory_address | ((1 as u64) << index),
                _ => panic!("Cannot happen")
            };

            fill_memory(memory,
                        &value,
                        index + 1,
                        &floating_mask,
                        flipped_memory_address);
        }
    } else { // Skip this round if we are not seeing a 1 for the floating mask
        fill_memory(memory,
                    &value,
                    index + 1,
                    &floating_mask,
                    current_memory_address);
    }
}

/// Will apply the mask_1 as an logical OR to the value
/// and also applies NOT mask_1 as an logical AND
fn apply_masks_to_value (value: u64, mask_1: &u64, mask_0: &u64) -> u64 {
    return (value | mask_1) & (!mask_0);
}

/// Will parse a line like mem[8] = 12
///
/// returns for the given example: `(8, 12)´
fn parse_line_memory(line: &str) -> (u64, u64) {

    // Build a regex which parses the mem[<memory>] = <value>
    // parts from the line
    let regex = Regex::new(r"mem\[(?P<memory>\d+)\] = (?P<value>\d+)").unwrap();
    let captures = regex.captures(line).expect(&format!("Couldn't parse instruction {}", line));

    return (captures["memory"].parse::<u64>().unwrap(),
            captures["value"].parse::<u64>().unwrap());
}

/// Will return three bit masks:
///
/// (1) positions of 1
/// (2) positions of 0
/// (3) positions of X
///
/// i.e., 100X1 => (10001, 01100, 00010)
/// only the 34 least significant bits are relevant
fn parse_line_mask(line: &str) -> (u64, u64, u64){
    let mut mask_1 : u64 = 0;
    let mut mask_0 : u64 = 0;
    let mut mask_x : u64 = 0;
    let mut current_exponent = 36;

    for char in line.bytes().skip(7) {
        if char == '1' as u8 {
            mask_1 |= (1 as u64) << (current_exponent - 1)
        } else if char == '0' as u8 {
            mask_0 |= (1 as u64) << (current_exponent - 1)
        } else if char == 'X' as u8 {
            mask_x |= (1 as u64) << (current_exponent - 1)

        }

        current_exponent -= 1;
    }

    return (mask_1, mask_0, mask_x);
}


/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}