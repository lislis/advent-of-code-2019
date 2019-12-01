mod lib;
mod days;

use crate::days::day01;

fn main() {
    println!("Day 01-1: {}", day01::part_1()); // 3249817
    println!("Day 01-2: {}", day01::part_2()); // 4871866
}
