use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_param(mode: i64, pos: usize, mem: &Vec<i64>) -> i64 {
	return match mode {
		0 => mem[mem[pos] as usize],
		1 => mem[pos],
		_ => mem[pos]
	};
}

fn run_program(program: &Vec<i64>, input1: i64, input2: i64) -> i64 {
	let mut pos: usize = 0;
	let mut stopped = false;
	let mut program_output: Vec<i64> = program.to_vec();
	let mut output = -1;
	let mut inputs = vec![input2, input1];

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
					program_output[output_pos as usize] = (param1 * param2) as i32;
				},
				3 => {
					let param1 = program_output[pos + 1];
					program_output[param1 as usize] = inputs.pop().unwrap();	
				},
				4 => {
					let param1 = get_param(param1_mode, pos + 1, &program_output);
					output = param1;
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
			};
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

	// println!("{:?}", output);

	return output;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let program: Vec<&str> = contents.split_terminator(',').collect();
    let program_nr: Vec<i64> = program.iter().map(|nr| nr.parse::<i64>().unwrap()).collect();

    let mut max = 0;
    let mut last_e2 = 0;
    for i in 5..10 {
    	for j in 5..10 {
    		for k in 5..10 {
    			for q in 5..10 {
    				for x in 5..10 {
    					if i != j  && i!=k && i!=q && i != x &&
    						j != k && j!= q && j!=x &&
    						k !=q && k !=x &&
    						q !=x {
    							println!("{:?}{:?}{:?}{:?}{:?}", i, j, k, q, x);
    						let a2 = run_program(&program_nr, i, last_e2);
    						let b2 = run_program(&program_nr, j, a2);
    						let c2 = run_program(&program_nr, k, b2);
    						let d2 = run_program(&program_nr, q, c2);
    						let e2 = run_program(&program_nr, x, d2);
    						last_e2 = e2;

    						if e2 > max {
    							max = e2;
    						}
    					}
    				}
    			}
    		}
    	}
    }

    println!("{:?}", max);
}