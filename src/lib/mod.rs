use std::fs::File;
use std::io::prelude::*;


fn read_file(path: &str) -> String {
   let mut file = File::open(path).expect("No file found");
   let mut content = String::new();
   file.read_to_string(&mut content).expect("Cannot read file");
   content
}

pub fn read_and_parse(path: &str) -> Vec<i64> {
    let content = read_file(path);    
    let lines = content.split("\n").collect::<Vec<&str>>();
    let values = lines.iter()
        .take(lines.len() -1)
        .map(|n| n.trim().parse::<i64>())
	.map(|n| match n {
		 Ok(num) => num,
		 Err(_err) => panic!("parsing int returned error")
	})
        .collect::<Vec<i64>>();
    values
}

pub fn read_and_parse_one_line(path: &str) -> Vec<usize> {
    let content = read_file(path);
    let nums = content.split(",").collect::<Vec<&str>>();
    let nums = nums.iter()
    	.map(|n| n.trim().parse::<usize>())
	.map(|n| match n {
		 Ok(num) => num,
		 Err(_err) => panic!("parsing int returned error")
	})
	.collect::<Vec<usize>>();
    nums
}