use crate::files::str_to_lines;
use regex::Regex;
use num::range;
use std::collections::HashMap;

/// Complex type which consists of a rule name and its two numeric boundaries as defined in the task
type Rule<'a> = (&'a str, (u64, u64), (u64, u64));

pub fn task1() {

    // Read the data
    let input = get_input_data();

    // And split it into their sections (rules, own ticket, nearby tickets)
    let parts = split_input_parts(input);
    let rules: Vec<Rule> = parts[0].iter().map(|r| parse_rule(r)).collect();
    let nearby_tickets: Vec<Vec<u64>> = parts[2].iter().map(|t| parse_ticket(t)).collect();

    let mut fail_sum: u64 = 0;
    // find all tickets which are not valid and sum the invalid columns thereof
    for nearby_ticket in nearby_tickets {
        fail_sum += get_invalid_rules_for_ticket(&nearby_ticket, &rules)
            .into_iter()
            .map(|index| nearby_ticket[index])
            .sum::<u64>();
    }

    println!("Sum of error values: {}", fail_sum);
}

pub fn task2() {

    // same as task [task1]
    let input = get_input_data();

    let parts = split_input_parts(input);
    let rules: Vec<Rule> = parts[0].iter().map(|r| parse_rule(r)).collect();
    let nearby_tickets: Vec<Vec<u64>> = parts[2].iter().map(|t| parse_ticket(t)).collect();
    let own_ticket: Vec<u64> = parse_ticket(&parts[1][0]);

    // Since our ticket may decide also in the assignment of columns to ticket entries
    // we will have to take that into account also
    let valid_tickets: Vec<Vec<u64>> = nearby_tickets
        .into_iter()
        .filter(|ticket| get_invalid_rules_for_ticket(ticket, &rules).len() == 0)
        .collect();


    // Stores a lookup table in the form of `rule -> [columns_where_rules_apply]´
    let mut rules_to_valid_columns: HashMap<String, Vec<usize>> = HashMap::new();

    // for each column find all matching rules (i.e, where all rows apply to rule)
    for column_num in range(0, own_ticket.len()) {
        for rule in rules.iter() {
            // println!("Check {} for column {}", rule_name, column_num);
            let rule_name = rule.0;

            let mut fits_all = true;
            for row_num in range(0, valid_tickets.len()) {
                let value_to_check = &valid_tickets[row_num][column_num];
                // println!("{:?}, {:?}", value_to_check, rule);
                if !applies(&rule, &value_to_check)  {
                    fits_all = false;
                    break;
                }
            }

            // Add column number to the valid columns of [rule_name]
            if fits_all {
                if !rules_to_valid_columns.contains_key(rule_name) {
                    rules_to_valid_columns.insert(String::from(rule_name), Vec::new());
                }
                let current_len = rules_to_valid_columns[rule_name].len();
                rules_to_valid_columns.get_mut(rule_name).unwrap().insert(current_len, column_num);
            }
        }
    }

    // Stores the final assignment in the form of `column -> rule_name´
    // i.e., assigns each column a matching rule
    let mut col_to_rule_name: HashMap<usize, String> = HashMap::new();

    // Loop until no rule is left or we reach an error condition
    while let Ok(rule_name) = find_rule_with_only_one_valid_column(&rules_to_valid_columns){

        // Which column does the rule belong to?
        let column_for_rule = rules_to_valid_columns[&rule_name][0];

        // Remember that this column was matched by the rule
        col_to_rule_name.insert(column_for_rule, rule_name.clone());

        // remove the found rule (we do not want to apply it twice)
        rules_to_valid_columns.remove(&rule_name);

        // remove also the column from all rules which also would match the column
        remove_column_from_lookup(&mut rules_to_valid_columns,
                                  column_for_rule);
    }

    // No we just need to find all columns which have `departure´ in their name
    let departure_columns: Vec<usize> = col_to_rule_name
        .into_iter()
        .filter(|entry| entry.1.starts_with("departure"))
        .map(|entry| entry.0).collect();

    // and multiply the values of those columns within the own ticket
    let res = departure_columns
        .into_iter()
        .fold(1 as u64, | prev, col| prev * own_ticket.get(col).unwrap());


    println!("Product of fields: {}", res);
}

/// checks if the [rule] matches [value]
fn applies(rule: &Rule, value: & u64) -> bool {
    let from1 = rule.1.0;
    let to1 = rule.1.1;
    let from2 = rule.2.0;
    let to2 = rule.2.1;

    return (*value >= from1 && *value <= to1) || (*value >= from2 && *value <= to2);
}

/// Will take the map `rule_name => [cols,...]´ and removes the [column_to_remove] everywhere
fn remove_column_from_lookup(lookup: &mut HashMap<String, Vec<usize>>, column_to_remove: usize) {
    for columns in lookup.values_mut() {
        let pos = columns.iter().position(|value| *value == column_to_remove);
        if pos.is_some() {
            columns.remove(pos.unwrap());
        }
    }
}

fn find_rule_with_only_one_valid_column (rules_to_valid_columns: &HashMap<String, Vec<usize>>) -> Result<String, ()> {
    for entry in rules_to_valid_columns.iter() {
        if entry.1.len() == 1 {
            return Ok(entry.0.clone());
        }
    }
    Err(())
}

/// Splits the input file into its parts where:
///     - first part are the rules,
///     - seconds part is the own ticket
///     - third part are the other peoples' tickets
fn split_input_parts(input: &str) -> Vec<Vec<String>> {
    let parts = input.split("\r\n\r\n");

    let mut line_parts: Vec<Vec<String>> = Vec::new();

    for (i, part) in parts.into_iter().enumerate() {
        let lines = str_to_lines(part);

        if i == 1 || i == 2 {
            line_parts.insert(i, lines.into_iter().skip(1).collect());
        } else {
            line_parts.insert(i,lines);
        }
    }

    return line_parts;
}


/// splits a line like `wagon: 35-898 or 907-957´ into ("wagon", (35, 898), (907, 857))
fn parse_rule(rule_line: &str) -> Rule {
    let re = Regex::new(r"^(?P<name>[a-zA-Z ]+): (?P<from1>\d+)-(?P<to1>\d+) or (?P<from2>\d+)-(?P<to2>\d+)$").expect("Invalid regex");
    let captures = re.captures(rule_line).expect(&format!("Couldn't apply regex to {}", rule_line));

    return (captures.name("name").unwrap().as_str(),
            (captures.name("from1").unwrap().as_str().parse::<u64>().unwrap(), captures.name("to1").unwrap().as_str().parse::<u64>().unwrap()),
            (captures.name("from2").unwrap().as_str().parse::<u64>().unwrap(), captures.name("to2").unwrap().as_str().parse::<u64>().unwrap()),
    );
}

/// Gets all indices of [ticket] where none of [rules] applies
fn get_invalid_rules_for_ticket(ticket: &Vec<u64>, rules: &Vec<Rule>) -> Vec<usize> {
    let mut failed_tickets : Vec<usize> = Vec::new();
    for (i, ticket_value) in ticket.iter().enumerate() {
        let mut found_a_valid_rule = false;
        for rule in rules {
            let from1 = rule.1.0;
            let to1 = rule.1.1;
            let from2 = rule.2.0;
            let to2 = rule.2.1;
            if (*ticket_value >= from1 && *ticket_value <= to1) || (*ticket_value >= from2 && *ticket_value <= to2)  {
                found_a_valid_rule = true;
                break;
            }
        }

        if !found_a_valid_rule {
            failed_tickets.insert(failed_tickets.len(), i);
        }
    }

    return failed_tickets;
}

/// Parses a line like `7,3,47´ to `vec![7,3,47]`
fn parse_ticket(rule_line: &str) -> Vec<u64> {
    return rule_line.split(",").into_iter().map(|num| num.parse::<u64>().unwrap()).collect();
}

/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}