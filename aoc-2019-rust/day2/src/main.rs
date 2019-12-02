use std::env;
use std::fs::File;
use std::io::prelude::*;

fn run_program(program: &Vec<i32>, noun: i32, verb: i32) -> i32 {
	let mut pos = 0;
	let mut stopped = false;
	let mut program_output: Vec<i32> = program.to_vec();

	program_output[1] = noun;
	program_output[2] = verb;

	program.iter().for_each(|_x| {
		if stopped { 
			return; 
		}

		if pos % 4 == 0 {
			match program_output[pos] {
				1 => {
					let output_pos = program_output[pos+3];
					program_output[output_pos as usize] = program_output[program_output[pos+1] as usize] + program_output[program_output[pos+2] as usize]
				},
				2 => {
					let output_pos = program_output[pos+3];
					program_output[output_pos as usize] = program_output[program_output[pos+1] as usize] * program_output[program_output[pos+2] as usize]
				},
				99 => stopped = true,
				_ => (),
			}
		}

		pos+=1;
	});

	return program_output[0];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let program: Vec<&str> = contents.split_terminator(',').collect();
    let program_nr: Vec<i32> = program.iter().map(|nr| nr.parse::<i32>().unwrap()).collect();

    (1..99).for_each(|noun| {
		(1..99).for_each(|verb| {
			if run_program(&program_nr, verb, noun) == 19690720 {
				println!("100*verb+noun = {:?}", 100*verb+noun);
			}
		});    	
    });

    println!("{:?}", run_program(&program_nr, 12, 2));
}