use std::collections::HashSet;
use std::collections::HashMap;
// task 1 IntoIter consumption review
fn main() {
    let v = vec![1,2,3];
    
    let it = v.iter();
    println!("{:?}", v);
    println!("{:?}", it);

    // task 2 implicit into_iter() consumption
    	
	for e in v.iter() {
		println!("{}", e);
	}
	println!("{:?}", v)
    // task 2..end..
}

// task 3 Conversion without loss of ownership
fn main() {
	let v = vec![1,2,3];
   
	let s: HashSet<i32> = v.clone().into_iter().collect();

	 println!("{:?}", v);
	println!("{:?}", s);
}
// task 3..end..

// task 4 Exercise: print elements and sum

fn main() {
	let hs = HashSet::from([1,2,3]);

	for e in &hs {
		println!("{}", e);
	}

	let sum: i32 = hs.iter().sum();

	println!("{}", sum);
}
// task 4..end..

// task 5 Exercise: vec to HashMap
fn main() {
	let v = vec![0,1,2];
	
	let hm: HashMap<usize, i32> = v.clone().into_iter().enumerate().collect();
	println!("v not consumed: {:?}", v);
	println!("HashMap: {:?}", hm);
}
 // task 5..end..