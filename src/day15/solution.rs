use std::collections::HashMap;
use std::borrow::BorrowMut;
use num::range;
use std::ops::Div;

pub fn task1(n_rounds: Option<usize>) {

    // Extract amount of rounds if we get some, we default to 2002 otherwise
    let rounds = n_rounds.unwrap_or(2020);
    let data = get_input_data();

    // creates our initial memory
    let initial_input = create_initial_memory(data);
    let mut memory = initial_input.0;
    let mut last_inserted_number = initial_input.1;

    // now do the turns
    for i in range(memory.keys().len() + 1,rounds + 1) {
        // println!("Turn: {}", i);
        let was_new = memory.get(&last_inserted_number).unwrap().len() == 1;
        if was_new {
            // Since the number is new, we insert a 0 at the end of the list of 0s
            insert_into_memory(&mut memory, 0, i as u64);
            // println!("Insert 0");
            last_inserted_number = 0;
        } else {
            // number is already inside -> we search the latest occurrence and the occurence before that
            let occurrences: &Vec<u64> = memory.get(&last_inserted_number).unwrap();
            let latest_occurrence = occurrences.get((occurrences.len() - 1)).unwrap();
            let before_latest_occurrence = occurrences.get((occurrences.len() - 2)).unwrap();

            let new_number =  latest_occurrence - before_latest_occurrence;
            /*println!(" \
                Last inserted: {}; \
                Latest occurence {}; \
                Before that {}; \
                New number {}", last_inserted_number, latest_occurrence, before_latest_occurrence, new_number);*/
            insert_into_memory(&mut memory, new_number, i as u64);

            last_inserted_number = new_number;

        }
    }

    println!("Last inserted number: {} (after {} rounds)", last_inserted_number, i);
}


pub fn task2() {
    // Task 2 is COMPLETELY the same as task i
    // Our solution is fast enough to do 15s (in release mode at least)
    task1(Some(30000000));
}

/// Inserts a value into the memory at point [value] with value [position_in_list]
///
/// creates a new list if the value was seen first
/// [position_in_list] is expected to be > than all other values of[position_in_list] before
fn insert_into_memory(memory: &mut HashMap<u64, Vec<u64>>, value: u64, position_in_list: u64) {

    // Check if this number does not exist yet
    if !memory.contains_key(&value) {
        // then we want to create a new entry list for that
        let mut new_vec : Vec<u64> = Vec::new();
        memory.insert(value, new_vec);
    }

    let t = memory.get_mut(&value).unwrap();
    t.push(position_in_list);
}

/// Loads the input file and persists the data into memory
///
/// returns the memory and the last inserted value
fn create_initial_memory(data: &str) -> (HashMap<u64, Vec<u64>>, u64) {
    let initial_numbers: Vec<u64> = data.split(',').into_iter().map(|c| c.parse::<u64>().expect("Could not parse input")).collect();
    let mut memory :HashMap<u64, Vec<u64>> = HashMap::new();
    let mut last_inserted_value: u64 = 0;
    for (i, number) in initial_numbers.iter().enumerate() {
        insert_into_memory(&mut memory, number.clone(), (i+1) as u64);
        last_inserted_value = number.clone();
    }

    return (memory, last_inserted_value);
}


/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}