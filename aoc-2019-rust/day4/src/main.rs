fn is_valid(pass: i32) -> bool {
	let mut nr = pass;
	let mut arr: Vec<i32> = vec![6; 0];

	while nr>0 {
		let n = nr % 10;
		arr.push(n);
		nr = nr/10;
	}

	arr.reverse();

	let original = arr.clone();
	let mut sorted = arr.clone();
	sorted.sort();

	let matching = original.iter().zip(&sorted).filter(|&(a, b)| a == b).count();


	if matching != 6 {
		return false;
	}	

	let mut same_pairs = 0;
	for i in 0..5 {
		if original[i] == original[i+1] {
			same_pairs = same_pairs + 1;
		} else if same_pairs > 1 {
			same_pairs = 0;
		} else if same_pairs == 1 {
			return true
		}
	}

	if same_pairs != 1  {
		return false;
	}

	return true;
	
}

fn main() {
	let mut pass = 0;
    for i in 307237..769058 {
    	if is_valid(i) {
    		pass = pass + 1;
    	}
    }

    println!("{:?}", pass);
}
