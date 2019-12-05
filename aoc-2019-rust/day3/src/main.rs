use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

fn parse_int(input: &str) -> Option<u32> {
    input
        .chars()
        .skip_while(|ch| !ch.is_digit(10))
        .take_while(|ch| ch.is_digit(10))
        .fold(None, |acc, ch| {
            ch.to_digit(10).map(|b| acc.unwrap_or(0) * 10 + b)
        })
}

fn get_inter(x1:i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Point {
	let denom = (y4 - y3)*(x2 - x1) - (x4 - x3)*(y2 - y1);

	if denom == 0 {
		return Point { x: 0, y: 0 };
	}

	let ua = ((x4 - x3)*(y1 - y3) - (y4 - y3)*(x1 - x3)) as f32 / denom as f32;
	let ub = ((x2 - x1)*(y1 - y3) - (y2 - y1)*(x1 - x3)) as f32/denom as f32;

	return Point{x: (x1 as f32 + ua * (x2 - x1) as f32) as i32, y: (y1 as f32 + ua * (y2 - y1) as f32) as i32};
}

fn do_intersect(a: i32, b: i32, c: i32, d: i32, p: i32, q: i32, r: i32, s: i32) -> bool {
	let det: i32;
	let gamma: f32;
	let lambda: f32;

	det = (c - a) * (s - q) - (r - p) * (d - b);

	if det == 0 {
		return false;
	}

	lambda = ((s - q) * (r - a) + (p - r) * (s - b)) as f32 / det as f32;
	gamma = ((b - d) * (r - a) + (c - a) * (s - b)) as f32 / det as f32;

	return (0.0 < lambda && lambda < 1.0) && (0.0 < gamma && gamma < 1.0);
}

fn compute_line(line: &Vec<&str>, mat: &mut Vec<Point>) {
	let mut last_point = Point { x: 0, y: 0};

	line.iter().for_each(|x| {
		let mut chars = x.chars();
		let direction = chars.next().unwrap();

		let last_x = last_point.x;
		let last_y = last_point.y;

		let point: Point = match direction {
			'R' => Point { x: last_x + parse_int(x).unwrap() as i32, y: last_y },
			'L' => Point { x: last_x - parse_int(x).unwrap() as i32, y: last_y },
			'U' => Point { x: last_x, y: last_y + parse_int(x).unwrap() as i32 },
			'D' => Point { x: last_x, y: last_y - parse_int(x).unwrap() as i32 },
			_ => Point{ x: 0, y: 0}
		};
		
		last_point.x = point.x;
		last_point.y = point.y;

		mat.push(point);
	});
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let program: Vec<&str> = contents.split_terminator('\n').collect();
    let first_line: Vec<&str> = program[0].split_terminator(',').collect();
    let second_line: Vec<&str> = program[1].split_terminator(',').collect();

    let mut path1: Vec<Point> = Vec::with_capacity(first_line.len());
    let mut path2 : Vec<Point> = Vec::with_capacity(second_line.len());

    compute_line(&first_line, &mut path1);
    compute_line(&second_line, &mut path2);

    let mut min = 0;

    for i in 0..first_line.len() - 1{
    	for j in 0..second_line.len() - 1 {
    		
    			let p1 = &path1[i];
    			let p11 = &path1[i+1];
    			let p2 = &path2[j];
    			let p22 = &path2[j+1];
    			if do_intersect(p1.x, p1.y, p11.x, p11.y, p2.x, p2.y, p22.x, p22.y) {
    				let inter_point = get_inter(p1.x, p1.y, p11.x, p11.y, p2.x, p2.y, p22.x, p22.y);

    				if inter_point.x.abs() + inter_point.y.abs() < min || min == 0 {
    					min = inter_point.x.abs() + inter_point.y.abs();
    				}
    			}
    		
    	}
    }

    println!("{:?}", min);

}