use std::collections::{HashSet, HashMap};
fn main() {
    // task 1  introduction to enumerate
	let v = vec![2,3,5,7,11,13,17];
	
	for (i, val) in v.into_iter().enumerate() {
		println!("{} {}", i, val);
	}
    // task 1..end..

    // task 2
    let v = vec![1, 3, 4, 5, 6];
    
    let result = first_even_index(v);

    println!("{:?}", result);
    // task 2..end..

    // task 3 iter wraps the collection with a reference, not the enumerate
    let v = vec![2,3,5,7,11];
	
	for (i, val) in v.iter().enumerate() {
		println!("{} {}", i, val);
		accept(i, val);
	}
    // task 3..end..

    // task 4
    let v = vec![10, 11, 12, 10, 14];
	let result = index_of_first_duplicate(v);
	println!("{:?}", result); 
    // task 4..end..

    // task 5
     let v = vec![0, 2, 2, 3];
		let result = equal_indices(v);

    println!("{:?}", result);
    // task 5..end..

    // task 6 Enumerate returns a tuple
    let v = vec![2,3,5,7,11];
	
	for t in v.into_iter().enumerate() {
		println!("{:?}", t);
		accept(t);
	}
    // task 6..end..

    // task 7 Exercise fill in the missing type
    	let v = vec![2,3,5,7,11];
	
	for t in v.iter().enumerate() {
		println!("{:?}", t);
		accept(t);
	}
    // task 7..end..

    // task 8 HashMap from Enumerate 1
     let v = vec![10,11,12];
    
    let hm: HashMap<usize, i32> = v.into_iter().enumerate().collect();
    println!("{:?}", hm);
    // task 8..end..

    // task 9 HashMap from Enumerate 2
     let v = vec![10,11,12];
    
    let hm: HashMap<usize, &i32> = v.iter().enumerate().collect();
    println!("{:?}", hm);
    // task 9..end..

    // task 10 Exercise: type of accept
    let v = vec![&1,&2,&3,&4];
	
	for (i, e) in v.iter().enumerate() {
	    accept(i, e);
	}
    // task 10..end..

    // task 11
     let v = vec![10,11,12];
    
    let result = mul_by_index(v);
    println!("{:?}", result);
    // task 11..end..
}

// task 2 Exercise: first even number’s index
pub fn first_even_index(v: Vec<i32>) -> Option<usize> {
    // your code here
    for (i, val) in v.into_iter().enumerate(){
         if val % 2 == 0 {
           return Some(i);
         } 
    }
    None
}
// task 2..end..

// task 3 iter wraps the collection with a reference, not the enumerate
fn accept(_i: usize, _v: &usize) {}
// task 3..end..

// task 4 First duplicate at index
pub fn index_of_first_duplicate(v: Vec<i32>) -> Option<usize> {
	// your code here
    let mut hs = HashSet::new();
    for (i, val) in v.iter().enumerate(){
       if hs.contains(val){
        return Some(i);
       }
       hs.insert(val);
    }
    None
}
// task 4..end..

// task 5 Equals own index
pub fn equal_indices(v: Vec<i32>) -> Vec<bool> {
    // your code here
    let mut b = Vec::new();
    for (i, val) in v.into_iter().enumerate(){
        if val == i as i32{
            b.push(true);
        }else {
            b.push(false);
        }
    } b
}
// task 5..end..

// task 6 Enumerate returns a tuple
fn accept(_t: (usize, i32)) {}
// task 6..end..

// task 7 Exercise fill in the missing type
fn accept(_v: (usize, &i32)) {}
// task 7..end..

// task 10 Exercise: type of accept
fn accept(_i:usize, _e:&i32) {}
// task 10..end..

// task 11 Inner Product with Index
pub fn mul_by_index(v: Vec<i32>) -> Option<i32> {
    let mut y = 0;
    // your code here
    if v.len()==0{
        return None;
    }
    for (i, val) in v.into_iter().enumerate(){
       y += (i as i32) * val ; 
    } 
    Some(y)

}
// task 11..end..