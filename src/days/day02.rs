use crate::lib::read_and_parse_one_line;

fn intcode(mut vec: Vec<usize>) -> Vec<usize> {
   let mut i = 0;

   loop {
   	match vec[i] {
	      1 => {
	      	let r = vec[i + 3];
	      	vec[r] = vec[vec[i + 1]] + vec[vec[i + 2]];
	      },
	      2 => {
	      	let r = vec[i + 3];
	     	vec[r] = vec[vec[i + 1]] * vec[vec[i + 2]];
	      },
	      99 => break,
	      _ => { }
	}

	i += 4;
   }

   vec
}

#[test]
fn test_intcode() {
   assert_eq!(intcode(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
   assert_eq!(intcode(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
   assert_eq!(intcode(vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
   assert_eq!(intcode(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
}

pub fn part_1() -> usize {
   let mut nums = read_and_parse_one_line("input-02.txt");
   nums[1] = 12;
   nums[2] = 2;
   let compute = intcode(nums);
   compute[0]
}

fn find_output(memory: Vec<usize>, needle: usize) -> (usize, usize) {
     let mut noun = 0;
     let mut verb = 0;
   
     for n in 0..=99 {
     	 for v in 0..=99 {
	     let mut mem = memory.clone();
	     mem[1] = n;
	     mem[2] = v;
	     let compute = intcode(mem);
	     if compute[0] == needle {
	     	noun = n;
		verb = v;
	     	break;
	     }
   	 }   
     }
     (noun, verb)
}

pub fn part_2() -> usize {
   let nums = read_and_parse_one_line("input-02.txt");
   let tup = find_output(nums, 19690720);
   100 * tup.0 + tup.1
}