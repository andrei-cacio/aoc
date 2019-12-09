use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_param(mode: i32, pos: usize, mem: &Vec<i32>) -> i32 {
	return match mode {
		0 => mem[mem[pos] as usize],
		1 => mem[pos],
		_ => mem[pos]
	};
}

fn run_program(program: &Vec<i32>, input: i32) -> Vec<i32> {
	let mut pos: usize = 0;
	let mut stopped = false;
	let mut program_output: Vec<i32> = program.to_vec();

	program.iter().for_each(|_x| {
		if stopped { 
			return; 
		}

		// if pos % 4 == 0 {
			let op_code = program_output[pos] % 100;
			let param1_mode = (program_output[pos] / 100) % 10;
			let param2_mode = program_output[pos] / 1000;
			let mut jumped_to = 0;

			// println!("{:?} -> {:?}", op_code, pos);

			match op_code {
				1 => {
					let param1 = get_param(param1_mode, pos + 1, &program_output);
					let param2 = get_param(param2_mode, pos + 2, &program_output);
					let output_pos = program_output[pos+3];
					program_output[output_pos as usize] = param1 + param2;
				},
				2 => {

					let param1 = get_param(param1_mode, pos + 1, &program_output);
					let param2 = get_param(param2_mode, pos + 2, &program_output);
					let output_pos = program_output[pos+3];
					program_output[output_pos as usize] = param1 * param2;
				},
				3 => {
					let param1 = program_output[pos + 1];
					program_output[param1 as usize] = input;	
				},
				4 => {
					let param1 = get_param(param1_mode, pos + 1, &program_output);
					println!("{:?}", param1);
				},
				5 => {
					let param1 = get_param(param1_mode, pos + 1, &program_output);
					let param2 = get_param(param2_mode, pos + 2, &program_output);
					if param1 != 0 {
						jumped_to = param2 as usize;
					}
				},
				6 => {
					let param1 = get_param(param1_mode, pos + 1, &program_output);
					let param2 = get_param(param2_mode, pos + 2, &program_output);
					if param1 == 0 {
						jumped_to = param2 as usize;
					}
				},
				7 => {
					let param1 = get_param(param1_mode, pos + 1, &program_output);
					let param2 = get_param(param2_mode, pos + 2, &program_output);
					let output_pos = program_output[pos+3];
					if param1 < param2 {
						program_output[output_pos as usize] = 1;
					} else {
						program_output[output_pos as usize] = 0;
					}
				},
				8 => {
					let param1 = get_param(param1_mode, pos + 1, &program_output);
					let param2 = get_param(param2_mode, pos + 2, &program_output);
					let output_pos = program_output[pos+3];
					if param1 == param2 {
						program_output[output_pos as usize] = 1;
					} else {
						program_output[output_pos as usize] = 0;
					}
				},
				99 => stopped = true,
				_ => (),
			}
		// }

		// println!("{:?}", jumped_to	);

		if op_code == 1 || op_code == 2 || op_code == 8 || op_code == 7  {
			pos += 4;
		} else if op_code == 3 || op_code == 4 {
			pos += 2;
		} else if op_code == 8 || op_code == 7 {
			pos += 4;
		} else if op_code == 5 || op_code == 6 {
			if jumped_to != 0 {
				pos = jumped_to
			} else {
				pos += 3;
			}
		}
	});

	return program_output.to_vec();
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

    run_program(&program_nr, 5);
}