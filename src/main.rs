mod lib;
mod days;

use crate::days::day01;
use crate::days::day02;

fn main() {
    println!("Day 01-1: {}", day01::part_1()); // 3249817
    println!("Day 01-2: {}", day01::part_2()); // 4871866
    println!("Day 02-1: {}", day02::part_1()); // 3716250
    println!("Day 02-2: {}", day02::part_2()); // 6472
}
