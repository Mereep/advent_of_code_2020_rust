#![feature(destructuring_assignment)]
#![feature(type_alias_impl_trait)]

pub mod files;
// #[path="day12/solution.rs"] mod day12;
//#[path="day13/solution.rs"] mod day13;
// #[path="day03/solution.rs"] mod day03;
//#[path="day02/solution.rs"] mod day02;
// #[path="day14/solution.rs"] mod day14;
// #[path="day15/solution.rs"] mod day15;
//#[path="day16/solution.rs"] mod day16;
#[path="day17/solution.rs"] mod day17;

fn main() {
    // To run a specific day, just uncomment the task you need
    // and also include the current day

    // day02::task1();
    // day02::task2();
    // day03::task2();
    // day12::task1();

    // day13::task1();
    // day13::task2();

    // day14::task1();
    // day14::task2();

    // day15::task1(None);
    // day15::task2();

    // day16::task1();
    // day16::task2();

    // day17::task1();
    day17::task2();
}