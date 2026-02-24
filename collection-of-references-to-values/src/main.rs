use std::collections::{HashSet, HashMap};
fn main() {
    // task 1 introduction to .copied()
	let v = vec![1,2,3];
	
	let result = get_max(&v);
	
	println!("{:?}", result);
    // task 1..end..

    // task 2
    let v = vec![&1,&2,&3];
	
	let result = convert(v);
	
	println!("{:?}", result);
    // task 2..end..

    // task 3 Exercise: convert vector of references to a set of owned values
    let v = vec![1, 2, 3];
    let v_ref: Vec<&i32> = v.iter().collect();

    let _hs: HashSet<i32> = v_ref.into_iter().copied().collect();
    // task 3..end..

    // task 4 Exercise: convert a set of references to a set of owned values
     let my_ref_set: HashSet<&i32> = HashSet::from([&1, &2, &3]);

    let my_owned_set: HashSet<i32> = my_ref_set.into_iter().copied().collect(); 
    foo(my_owned_set);
    // task 4..end..

    // task 5 .iter() and .copied() as opposites
     let v: Vec<i32> = vec![1,2,3];
    
    // convert Vec<i32> into Vec<&i32>
    let v_ref: Vec<&i32> =v.iter().collect();
    
    // convert Vec<&i32> into Vec<i32>
    let _w: Vec<i32> = v_ref.into_iter().copied().collect();
    // task 5..end..

    // task 6 Converting an Option<&i32> to Option<i32>
     // o_ref is Option<&i32>
    let o_ref: Option<&i32> = Some(&42);

    // o is an Option<i32>
    let _o: Option<i32> = o_ref.copied();
    // task 6..end..

    // task 7 Exercise: .get() on a vector
     let v = vec![1, 2, 3];

    let first = v.get(0).copied();
    accept(first);
    // task 7..end..

    // task 8
    let v = vec![&(1,10),&(2,20),&(3,30)];
    
    let result = convert(v);
    
    println!("{:?}", result);
    // task 8..end..
}

// task 1 introduction to .copied()
pub fn get_max(v: &Vec<i32>) -> Option<i32> {
    let maxx = v.into_iter().max();
    maxx.copied()
}
// task 1..end..

// task 2 Converting a vector of reference to a vector of owned copy types
pub fn convert(v: Vec<&i32>) -> Vec<i32> {
    //let ret: Vec<i32> = vec![];
    let ret = v.into_iter().copied().collect();
    ret
}
// task 2..end..

// task 4 Exercise: convert a set of references to a set of owned values
pub fn foo(_hs: HashSet<i32>) {}
// task 4..end..

// task 7 Exercise: .get() on a vector

// <do not edit>
pub fn accept(_e: Option<i32>) {
    println!("success!");
}
// </do not edit>
// task 7..end..

// task 8 Exercise: vector of reference tuples to HashMap
pub fn convert(v: Vec<&(i32, i32)>) -> HashMap<i32, i32> {
	v.into_iter().copied().collect()
}
// task 8..end..