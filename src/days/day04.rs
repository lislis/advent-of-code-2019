use std::collections::HashSet;
use std::collections::HashMap;

/// 206938-679128


fn check_password(num: &usize) -> bool {
   let digits: Vec<_> = num.to_string()
			   .chars()
			   .map(|d| d.to_digit(10).unwrap() as usize)
			   .collect();

   // what about the highest value?
   for (k, v) in digits.iter().enumerate() {
       if k != 0 {
         if v < &digits[k-1_usize] {
       	   return false;
         }
       }
   }

   // do we have double digits in adjacent positions?
   let mut flag: bool = false;
   for (k, v) in digits.iter().enumerate() {
       if k == 0 {
            if digits[k+1_usize] == *v {
	    flag = true;
	    }
	} else if k == 5 {
	  if digits[k-1_usize] == *v {
	      flag = true;
	   }
	} else {
	  if digits[k+1_usize] == *v || digits[k-1_usize] == *v {
	     flag = true;
	  }
        }
   }
   if !flag {
     return false;
   }
   true
}

#[test]
fn test_check_password() {
  assert!(check_password(&111111_usize));
  assert_eq!(check_password(&223450_usize), false);
  assert_eq!(check_password(&123789_usize), false);
  assert_eq!(check_password(&222224_usize), true);
}


pub fn part_1() -> usize {
   let opt: Vec<usize> = (206938..=679128)
       .collect::<Vec<usize>>()
       .into_iter()
       .filter(|n| check_password(n))
       .collect();

   opt.len()
}