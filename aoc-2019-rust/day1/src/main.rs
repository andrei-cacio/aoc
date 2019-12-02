use std::env;
use std::fs::File;
use std::io::prelude::*;

fn compute_fule(module: i32) -> i32 {
	let fule = module / 3 - 2;

	if fule / 3 - 2 < 0 {
		return fule;
	}
	return fule + compute_fule(fule);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let v: Vec<&str> = contents.split_terminator('\n').collect();
    let nr_v: Vec<i32> = v.iter().map(|nr| nr.parse::<i32>().unwrap()).collect();
    
    let first_sum = nr_v.iter().fold(0, |acc, x| acc + x/3-2);
    let second_sum = nr_v.iter().fold(0, |acc, x| acc + compute_fule(*x));
    

    println!("day1: {:?}", first_sum);
    println!("day1: {:?}", second_sum);
}