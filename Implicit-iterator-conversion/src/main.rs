use std::collections::HashSet;

fn main() {
    // task 1 Implicit into_iter in loops
	let hs = HashSet::from([1,2,3]);
	
	for e in hs {
		println!("{}", e);
	}
    // task 1..end..

    // task 2 For loop consumption
    let hs = HashSet::from([1,2,3]);
	
	for e in hs {
		println!("{}", e);
	}
	
    // task 2..end..

    // task 3 Exercise: remove unnecessary into_iter
    let v = vec![1,2,3];
    
    for e in v {
        println!("{}", e);
    }
    // task 3..end..

    // task 4 Exercise: implicit into_iter
    let s1 = HashSet::from([1,2,3]);
	let s2 = HashSet::from([2,3,4,5]);
	
	let result = merge(s1, &s2);
	
	println!("{:?}", result);
    // task 4..end..

    // task 5 Exercise: for loop on a reference to a collection
    let v = vec![1,2,3];
	let w = &v;
	
	for e in w {
		accept(e);
	}
    // task 5..end..

    // task 6 Exercise: merge if even
    let s1 = HashSet::from([1,2,3]);
	let s2 = HashSet::from([2,3,4,5]);
	
	let result = merge(s1, &s2);
	
	println!("{:?}", result);
    // task 6..end..
}

// task 4 Exercise: implicit into_iter
pub fn merge(mut s1: HashSet<i32>, s2: &HashSet<i32>) -> HashSet<i32> {
	for i in s2 {
        s1.insert(*i);
    }
    s1
}
// task 4..end..

// task 5 Exercise: for loop on a reference to a collection
fn accept(_v: &i32) {}
// task 5..end..

// task 6 Exercise: merge if even
pub fn merge(mut s1: HashSet<i32>, s2: &HashSet<i32>) -> HashSet<i32> {
	// your code here
    for i in s2 {
        if *i % 2 == 0 {
            s1.insert(*i);
        }
    }s1
}
// task 6..end..