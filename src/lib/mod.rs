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

pub fn read_and_parse_cables(path: &str) -> Vec<Vec<(char, usize)>> {
    let content = read_file(path);
    let coords = convert_to_tuples(content);
    coords
}


fn convert_to_tuples(content: String) -> Vec<Vec<(char, usize)>> {
    let coords = content.split("\n").collect::<Vec<&str>>();
    let coords = coords.into_iter()
    	.take(2)
    	.map(|n| n.split(',').collect())
	.map(|n: Vec<&str>| n.into_iter()
	    .map(|m| {
	        let mut chars = m.chars().collect::<Vec<char>>();
	   	let c = chars[0];
	    	chars.remove(0);

                let mut string = String::new();
    		for c in chars {
		    string.push(c);
    		}
		
	        let num = string.parse::<usize>();
		let num = match num {
	    	    Ok(number) => number,
		    Err(_err) => panic!("cannot parse input")
	    	};
		(c, num)
	    }).collect())
	.collect();
    coords
}
