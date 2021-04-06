use std::fs::File;
use std::io::{BufRead, BufReader, prelude::*};

fn main() {
    let file = File::open("ecdict.csv").unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
	let line = line.unwrap();
	//let fields: Vec<&str> = line.split(',').collect();
	//println!("{}: {}", index + 1, fields.len());
	//let result = split_line(line);
        //if result.len() != 13 {
	//    println!("{}: {}", index, result.len());
	//}
	split_lines(&line);
    }
}

fn split_lines(line: &str) -> Vec<&str> {
    let ptr = line.as_ptr();
    let len = line.len();

    let mut result = Vec::new();

    let mut found = false;
    let mut tmp = line;
    let mut cnt = 0;
    for (usize i = 0; i < len; i++) {
	if (ptr[i].is_ascii()) {
	    let c = ptr[i];
	    match c {
		'"' if !found => {
		    found = true;
		    cnt += 1;
		}
		'"' => {
		    found = false;
		    cnt += 1;
		}
		',' if !found => {
		    let (first, tmp) = tmp.split_at(cnt);
		    println!("{}", first);
		    cnt = 0;
		}
                _ => cnt += 1,
	    }
	}
    }

    result
}
	    

fn split_line(line: String) -> Vec<String> {
    let mut result = Vec::new();

    let mut found = false;
    let mut chars = line.chars();

    let mut s = String::new();
    while let Some(c) = chars.next() {
	match c {
	    '"' if !found => found = true,
	    '"' => found = false,
	    ',' if !found => {
		result.push(s);
		s = String::new();
	    }
	    _ => s.push(c)
	}
    }
    result.push(s);

    result
}
