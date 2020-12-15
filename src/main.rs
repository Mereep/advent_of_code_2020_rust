#![feature(destructuring_assignment)]

pub mod files;
// #[path="day12/solution.rs"] mod day12;
//#[path="day13/solution.rs"] mod day13;
// #[path="day03/solution.rs"] mod day03;
// #[path="day14/solution.rs"] mod day14;
#[path="day15/solution.rs"] mod day15;

fn main() {
    // To run a specific day, just uncomment the task you need
    // and also include the current day

    // day03::task2();
    // day12::task1();

    // day13::task1();
    // day13::task2();

    // day14::task1();
    // day14::task2();

    day15::task1(None);
    day15::task2();
}