#![feature(iterator_fold_self)]

pub mod files;
// #[path="day12/solution.rs"] mod day12;
//#[path="day13/solution.rs"] mod day13;
#[path="day03/solution.rs"] mod day03;

fn main() {
    // To run a specific day, just uncomment the task you need
    // and also include the current day

    day03::task2();
    // day12::task1();

    // day13::task1();
    // day13::task2();
}