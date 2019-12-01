use crate::lib::read_and_parse;

pub fn part_1() -> i64 {
    let values = read_and_parse("input-01.txt");

    let module_fuel = |n| {
       let num = n / 3; // i64 rounds down automatically for us
       let num = num - 2;
       num  
     };

    let sum: i64 = values.iter()
       .map(module_fuel)
       .sum();
   sum
}

fn calc_fuel(n: i64) -> i64 {
   let num = n / 3; // i64 rounds down automatically for us
   let num = num - 2;
   num
}


fn calc_fuel_deps(n: i64) -> i64 {
   let mut f = calc_fuel(n);
   let mut acc = f;

   loop {
   	f  = calc_fuel(f);
	if f <= 0 {
	  break;
	} else {
	  acc += f;
	}
   }
   acc
}

#[test]
fn test_calc_fuel_deps() {
   assert_eq!(calc_fuel_deps(14), 2);
   assert_eq!(calc_fuel_deps(1969), 966);
   assert_eq!(calc_fuel_deps(100756), 50346);
}

pub fn part_2() -> i64 {
    let values = read_and_parse("input-01.txt");

    let fuels = values.iter()
		      .map(|n| calc_fuel_deps(*n))
    	      	      .sum();
    fuels
}
