use crate::files::str_to_lines;

pub fn task1() {
    let data = str_to_lines(get_input_data());

    let n_lines = data.len();
    let n_cols = data[0].len();

    // steps in x (right) direction and y (down)-direction
    let dx = 3;
    let dy = 1;

    // current position
    let mut pos_x = 0;
    let mut pos_y = 0;

    // count of hit trees
    let mut n_trees = 0;

    // We step until we are out of the y span of the wood
    while pos_y < n_lines {
        if data[pos_y].as_bytes()[pos_x] == '#' as u8  {
            n_trees += 1;
        }

        // move one step
        pos_y += dy;

        // in x-direction we pretend that the world is repeating
        pos_x = (pos_x + dx) % n_cols;
    }

    println!("Trees hit: {}", n_trees);
}

// Basically the same as [task1]
pub fn task2() {
    let data = str_to_lines(get_input_data());

    let n_lines = data.len();
    let n_cols = data[0].len();

    // steps in x (right) direction and y (down)-direction
    // here is the difference, we don't only try one solution but a list of solutions
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    // the results for each attempt are collected here
    let mut n_trees_hit: Vec<u64> = Vec::new();

    for slope in slopes.iter() {
        let dx = slope.0;
        let dy = slope.1;

        // current position
        let mut pos_x = 0;
        let mut pos_y = 0;

        // count of hit trees
        let mut n_trees: u64 = 0;

        // We step until we are out of the y span of the wood
        while pos_y < n_lines {
            if data[pos_y].as_bytes()[pos_x] == '#' as u8  {
                n_trees += 1;
            }

            // move one step
            pos_y += dy;

            // in x-direction we pretend that the world is repeating
            pos_x = (pos_x + dx) % n_cols;
        }

        n_trees_hit.push(n_trees);
    }


    // And the product of those hits per run is the solution for that task
    println!("Trees hit total: {}, Product (Solution): {}",
             (&n_trees_hit).into_iter().sum::<u64>(),
             (&n_trees_hit).into_iter().product::<u64>()
             );

}

/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}
