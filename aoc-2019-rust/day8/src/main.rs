use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let img_data: Vec<u8> = contents
    	.chars()
    	.collect::<Vec<char>>()
    	.iter()
    	.map(|char| char.to_digit(10).unwrap() as u8)
    	.collect();

    let size = 6*25;
    let mut index = 1;

    let mut min_zeros = 0;
    let mut zeros = 0;
    let mut prod = 1;
    let mut ones = 0;
    let mut twos = 0;
    img_data.iter().for_each(|pixel| {
    	if *pixel == 0 {
    		zeros +=1;
    	} else if *pixel == 1 {
    		ones += 1;
    	} else if *pixel == 2 {
    		twos += 1;
    	}

    	if index % size == 0 {
    		if min_zeros == 0 || min_zeros > zeros {
    			min_zeros = zeros;
    			prod = ones * twos;
    		} 

    		ones = 0;
    		twos = 0;
    		zeros = 0;
    	} 
    	index += 1;
    });

    println!("");
    println!("{:?}", prod);
}