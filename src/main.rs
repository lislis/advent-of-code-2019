mod lib;

use crate::lib::*;


// 01-1 3249817


fn main() {
   let values = read_and_parse("input-01.txt");
   println!("{:?}", values);

   let module_fuel = |n| {
       let num = n / 3; // i64 rounds down automatically for us
       let num = num - 2;
       num  
   };

   let sum: i64 = values.iter()
       .map(module_fuel)
       .sum();
    println!("Puzzle 01-1: {}", sum);
}
