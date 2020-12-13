use crate::files::{str_to_lines};
use std::iter::Map;
use std::collections::HashMap;
use num::integer::lcm;


pub fn task1() {
    let file_contents = get_input_data();

    let lines = str_to_lines(file_contents);

    // This is the earliest timestamp we can take off with a bus
    let arrival_time = lines[0].parse::<u64>().expect(
        "Couldn't parse arrival time (first line)");

    // (i) we split the arriving times at ,
    // (ii) filter all "x" (not available)
    // (iii) we transform the string to an actual number
    let available_bus_ids: Vec<u64> = lines[1]
        .split(',')
        .into_iter()
        .filter(|char| *char != "x")
        .map(|number_str| number_str.parse::<u64>().expect(
            &format!("Couldn't parse number {}", number_str)))
        .collect();

    // Stores the bus id together with the time we would have to wait for it
    let mut earliest_line: (u64, u64) = (
        available_bus_ids[0],
        get_waiting_time(arrival_time, available_bus_ids[0]));

    // we iterate over all busses
    for bus_id in available_bus_ids.into_iter().skip(1) {

        // ... and calculate how long we wait for it
        let waiting_time = get_waiting_time(arrival_time, bus_id);

            /*println!("Waiting for line id: {}, waiting time: {}; Result: {}",
                     bus_id,
                     waiting_time,
                     bus_id * waiting_time
              );
             */

        // ... if this waiting time is less than for the other busses until here
        // we remember this bus id with its waiting time
        if earliest_line.1 > waiting_time {
            earliest_line = (bus_id, waiting_time);
        }

    }

    println!("Solution Day13.1: Earliest line id: {}, waiting time: {}; Result: {}",
             earliest_line.0,
             earliest_line.1,
             earliest_line.0 * earliest_line.1
    );

}

pub fn task2() {
    let file_contents = get_input_data();
    let lines = str_to_lines(file_contents);


    // We read al busses from the input (split by `,´)
    let bus_ids: Vec<&str> = lines[1]
        .split(',')
        .into_iter()
        .collect();

    // The basic idea is to use the chinese remainder theorem to find a time
    // where the different busses are one minute apart each (while interval 'x') does not matter)

    // Save the highest bus id (only relevant for the naive solution
    // since it will be our smallest increment for brute force
    let mut highest_bus_id: u64 = 0;

    // Build a lookup which is points from bus_id => position (index) within the input list
    // s.t. [10, 3, x, 5] would be (10 => 0, 3=>1, 5=>3)
    let mut offsets = HashMap::<u64, u64>::new();
    for (i, bus_id) in bus_ids.into_iter().enumerate() {
        if bus_id != "x" {
            let current_bus_id: u64 = bus_id.parse::<u64>().expect("Couldn't parse bus id");
            if current_bus_id > highest_bus_id {
                highest_bus_id = current_bus_id;
            }
            offsets.insert(current_bus_id,
                           i as u64);
        }
    }

    // We will use the chinese remainder theorem
    // this finds a `a´ where `a = x_n mod m_n´ holds for all x_n and m_n
    let mut moduli: Vec<i128> = Vec::new();         // stores all the m_n
    let mut remainders: Vec<i128> = Vec::new();     // stores all the x_n

    // each bus should come each separated by one minute times their list position
    // so the moduli is the bus_id (=interval) and the remainder is the list position
    for (bus_id, offset) in (&offsets).into_iter() {
        moduli.push((*bus_id) as i128);
        remainders.push((*offsets.get(bus_id).unwrap()) as i128);
    }

    // Calculate the chinese remainder
    let a = chinese_remainder(
        remainders.as_slice(),
        moduli.as_slice(),
    );

    // bad news: the chinese reminder doesn't necessrily find the smallest solution
    // However if we find the least common multiply (lcm) we can reduce the solution
    // to its smallest version bei calculating lcm % a
    let lcm = (&moduli).into_iter().fold(
        moduli[0], |a, b| lcm(a.clone(), b.clone()));

    println!("Smallest time offset: {:?}", lcm % a.unwrap());
    println!("LCM: {:?}", lcm);
    println!("Modulus {:?}", moduli);
    println!("Remainders {:?}", remainders);

    /*
    // Naive solution (Does calculate way too slow for bigger inputs)
    // Will likely take a day or so to find it

    let mut run_id: u64 = 1;
    let offset_of_highest_bus_id = offsets.get(&highest_bus_id)
        .expect("Couldn't find highest bus id in map");

    loop {
        let current_offset = run_id * highest_bus_id - *offset_of_highest_bus_id;

        let mut all_offsets_are_0 = true;
        for (bus_id, offset) in (&offsets).into_iter() {
            all_offsets_are_0 &= ((current_offset + offset) % bus_id) == 0;
        }

        if all_offsets_are_0 {
            println!("Current offset {}", current_offset);
            break;
        }

        if run_id % 10000 == 0 {
            println!("Current offset (run #{}): {}", run_id, current_offset)
        }

        run_id += 1;
    }
    */
}

/// calculates the time in minutes we would have to wait (beginning from [arrival_time] until
/// the next bus with id [bus_id] would arrive.
///
/// Remember that that the [bus_id] is also the driving interval of the bus
fn get_waiting_time(arrival_time: u64, bus_id: u64) -> u64 {
    let div = arrival_time.div_euclid(bus_id);
    let rest_to_arrival_time = arrival_time - div * bus_id;
    bus_id - rest_to_arrival_time
}

/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}



/// Chinese reminder theorem calculations
fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}