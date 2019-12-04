use std::collections::HashMap;
use std::collections::HashSet;
use crate::lib::read_and_parse_cables;

type Inst = (char, usize);
type Coord = (i64, i64);
type Cable = HashMap<Coord, usize>;


fn follow_cable(cable: &Vec<Inst>) -> Cable {
   let mut pos = (0_i64, 0_i64);
   let mut hm = Cable::new();
   for c in cable.iter() {
     match c {
       ('U', y) => {
       	 for i in 0..*y {
	     let p = (pos.0, pos.1 + i as i64);
	     hm.entry(p).or_insert(0);
	 }
         pos = (pos.0, pos.1 + *y as i64)
       },
       ('D', y) => {
          for i in 0..*y {
	     let p = (pos.0, pos.1 - i as i64);
	     hm.entry(p).or_insert(0);
	  }
          pos = (pos.0, pos.1 - *y as i64)
	},
       ('R', y) => {
          for i in 0..*y {
	     let p = (pos.0 + i as i64, pos.1);
	     hm.entry(p).or_insert(0);
	  }
       	  pos = (pos.0 + *y as i64, pos.1)
	},
       ('L', y) => {
       	  for i in 0..*y {
	     let p = (pos.0 - i as i64, pos.1);
	     hm.entry(p).or_insert(0);
	  }   
       	  pos = (pos.0 - *y as i64, pos.1)
	}
       _ => {}
     };
   };
   hm
}

fn follow_all_cables(cables: Vec<Vec<Inst>>) -> HashSet<Coord> {
   let mut cable1 = follow_cable(&cables[0]);
   let mut matches: HashSet<Coord> = HashSet::new();
   let mut pos = (0_i64, 0_i64);
   
   for c in cables[1].iter() {
     match c {
       ('U', y) => {
       	 for i in 0..*y {
	     let p = (pos.0, pos.1 + i as i64);
	     if cable1.contains_key(&p) {
	     	matches.insert(p);
	     }
	 }
         pos = (pos.0, pos.1 + *y as i64)
       },
       ('D', y) => {
          for i in 0..*y {
	     let p = (pos.0, pos.1 - i as i64);
	     if cable1.contains_key(&p) {
	     	matches.insert(p);
	     }
	  }
          pos = (pos.0, pos.1 - *y as i64)
	},
       ('R', y) => {
          for i in 0..*y {
	     let p = (pos.0 + i as i64, pos.1);
	     if cable1.contains_key(&p) {
	     	matches.insert(p);
	     }
	  }
       	  pos = (pos.0 + *y as i64, pos.1)
	},
       ('L', y) => {
       	  for i in 0..*y {
	     let p = (pos.0 - i as i64, pos.1);
	     if cable1.contains_key(&p) {
	     	matches.insert(p);
	     }
	  }   
       	  pos = (pos.0 - *y as i64, pos.1)
	}
       _ => {}
     };
   }
   matches
}

fn get_distances(ints: HashSet<Coord>) -> Vec<i64> {
   let origin: Coord = (0, 0);
   let dists = ints.iter()
		   .map(|n| (n.0 - origin.0).abs() + (n.1 - origin.1).abs())
		   .collect();
   dists
}

pub fn part_1() -> i64 {
   let instructions = read_and_parse_cables("input-03.txt");
   let intersections = follow_all_cables(instructions);
   let mut distances = get_distances(intersections);

   distances.sort();
   distances.remove(0);  // we still have 0,0 in there ....
   distances[0]
}






fn follow_cable_count_steps(cable: &Vec<Vec<Inst>>) -> Cable {
   let mut pos = (0_i64, 0_i64);
   let mut steps = 0_usize;
   
   let mut hm = Cable::new();
   for c in cable[0].iter() {
     match c {
       ('U', y) => {
       	 for i in 0..*y {
	     let p = (pos.0, pos.1 + i as i64);
	     let s = steps + i;
	     hm.entry(p).or_insert(s);
	 }
         pos = (pos.0, pos.1 + *y as i64);
	 steps += *y;
       },
       ('D', y) => {
          for i in 0..*y {
	     let p = (pos.0, pos.1 - i as i64);
	     let s = steps + i;
	     hm.entry(p).or_insert(s);
	  }
          pos = (pos.0, pos.1 - *y as i64);
	  steps += *y;
	},
       ('R', y) => {
          for i in 0..*y {
	     let p = (pos.0 + i as i64, pos.1);
	     let s = steps + i;
	     hm.entry(p).or_insert(s);
	  }
       	  pos = (pos.0 + *y as i64, pos.1);
      	  steps += *y;
	},
       ('L', y) => {
       	  for i in 0..*y {
	     let p = (pos.0 - i as i64, pos.1);
	     let s = steps + i;
	     hm.entry(p).or_insert(s);
	  }   
       	  pos = (pos.0 - *y as i64, pos.1);
	  steps += *y;
	}
       _ => {}
     };
   };

   pos = (0, 0);
   steps = 0;
   let mut matches = Cable::new();


   for c in cable[1].iter() {
     match c {
       ('U', y) => {
       	 for i in 0..*y {
	     let p = (pos.0, pos.1 + i as i64);
	     let s = steps + i;
	     if hm.contains_key(&p) {
	     	let prev_s = hm.get(&p).unwrap();
	     	matches.entry(p).or_insert(prev_s + s);
	     }
	 }
         pos = (pos.0, pos.1 + *y as i64);
	 steps += *y;
       },
       ('D', y) => {
          for i in 0..*y {
	     let p = (pos.0, pos.1 - i as i64);
	     let s = steps + i;
	     if hm.contains_key(&p) {
	     	let prev_s = hm.get(&p).unwrap();
	     	matches.entry(p).or_insert(prev_s + s);
	     }
	  }
          pos = (pos.0, pos.1 - *y as i64);
	  steps += *y;
	},
       ('R', y) => {
          for i in 0..*y {
	     let p = (pos.0 + i as i64, pos.1);
	     let s = steps + i;
	     if hm.contains_key(&p) {
	     	let prev_s = hm.get(&p).unwrap();
	     	matches.entry(p).or_insert(prev_s + s);
	     }
	  }
       	  pos = (pos.0 + *y as i64, pos.1);
	  steps += *y;
	},
       ('L', y) => {
       	  for i in 0..*y {
	     let p = (pos.0 - i as i64, pos.1);
	     let s = steps + i;
	     if hm.contains_key(&p) {
	     	let prev_s = hm.get(&p).unwrap();
	     	matches.entry(p).or_insert(prev_s + s);
	     }
	  }   
       	  pos = (pos.0 - *y as i64, pos.1);
	  steps += *y;
	}
       _ => {}
     };
   }

  matches
}




pub fn part_2() -> usize {
    let instructions = read_and_parse_cables("input-03.txt");

    let matches = follow_cable_count_steps(&instructions);

    let mut steps = matches.values().map(|n| *n).collect::<Vec<usize>>();
    steps.sort();
    steps.remove(0);
    
    steps[0]
}