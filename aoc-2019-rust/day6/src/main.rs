use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_distance_to_parent(node: &str, parent: &str, map: &HashMap<&str, &str>) -> i32 {
	let mut distance = 0;
	let mut found = false;
	let mut node_parent = map.get(node).unwrap();

	while !found {
		distance = distance + 1;
		if *node_parent == parent {
			found = true;
		} else {
			node_parent = map.get(node_parent).unwrap();
		}
	}

	return distance;
}


fn find_common_parent(node1: &str, node2: &str, map: &HashMap<&str, &str>) -> String {
	let mut found = false;
	let mut parent = "";

	let mut node1_parent = map.get(node1).unwrap();

	while !found {
		
		let mut node2_parent = map.get(node2).unwrap();
		
		let mut node2_has_parent = true;
		while node2_has_parent && !found {
			if node1_parent == node2_parent {
				parent = node1_parent;
				found = true;
			}

			node2_has_parent = match map.get(node2_parent) {
				Some(p) => true,
				None => false,	
			};

			if node2_has_parent {
				node2_parent = map.get(node2_parent).unwrap();
			}
		}

		node1_parent = map.get(node1_parent).unwrap()
	}

	return parent.to_owned();
}

fn get_count(map: &HashMap<&str, &str>) -> i32 {
	let mut c = 0;

	for key in map.keys() {
		let mut parent = map.get(key).unwrap();
		let mut done = false;
		
		while !done {
			done = match map.get(parent) {
				Some(p) => false,
				None => true,
			};

			if !done {
				parent = map.get(parent).unwrap();
			}

			c = c+1;
		}
	}

	return c;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let map: Vec<&str> = contents.split_terminator('\n').collect();
    let mut orbit_map = HashMap::new();

    map.iter().for_each(|i| {
    	let orbit_data: Vec<&str> = i.split_terminator(')').collect();

    	orbit_map.insert(orbit_data[1], orbit_data[0]);
    });
    
    println!("{:?}", get_count(&orbit_map));

    let common_parent = find_common_parent("YOU", "SAN", &orbit_map);
    let you_dist = get_distance_to_parent("YOU", &common_parent, &orbit_map) - 1;
    let san_dist = get_distance_to_parent("SAN", &common_parent, &orbit_map) - 1;
    let min_dist = you_dist + san_dist;

    println!("{:?}", min_dist);
}