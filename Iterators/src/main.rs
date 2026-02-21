// task 1 Size of HashSet
use std::collections::HashSet;

fn main() {
    // task 1 Size of HashSet
    let mut s1 = HashSet::new();
    s1.insert(1);
    s1.insert(2);
    s1.insert(3);
    
    let l = size_of_set(&s1);
    println!("{}", l);
    // task 1..end..
    
    // task 2 HashSet Iteration
    let s = HashSet::from([2,4,8,10]);
 let s_iter = s.into_iter();	
	for item in s_iter {
		println!("{}", item);
	}
    // task 2..end..

    // task 3 Vector Iteration
    let v = Vec::from([2,4,8,10]);
let v_iter = v.into_iter();	
	for item in v_iter {
		println!("{}", item);
	}
    // task 3..end..

    // task 4 Vector Iteration with Clone
     let v = Vec::from([2,4,8,10]);
    let s = v.clone();
    let v_iter = s.into_iter();
	for item in v_iter {
		println!("{}", item);
	}
	
	println!("{:?}", v);
    // task 4..end..    

    // task 5 HashSet Collection from Vector
    	let v = Vec::from([2,4,8,10]);
    let v_iter = v.into_iter();
	let s: HashSet<i32> = v_iter.collect();
    println!("{:?}", s); 
    // task 5..end..

    // task 6 HashSet to Vector Conversion
    let mut s = HashSet::new();
	s.insert(1);
	s.insert(2);
	s.insert(3);
	let v_iter = s.into_iter();
	let v: Vec<i32> = v_iter.collect();
    println!("{:?}", v);
    // task 6..end..

    // task 7 HashSet to HashSet Conversion
    let mut s = HashSet::new();
	s.insert(1);
	s.insert(2);
	s.insert(3);
    let v_iter = s.into_iter();
	let v: HashSet<i32> = v_iter.collect();
    println!("{:?}", v);
    // task 7..end..

    // task 8  Vector Deduplication with HashSet
    let v = vec![1, 2, 2, 3, 4, 4, 4, 5, 1];
  let deduped_v = dedup(v);
  println!("{:?}", deduped_v);
  // task 8..end..

    // task 9 Filtering HashSet Elements
    let s: HashSet<i32> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let odds_s = remove_evens(s);
    println!("Original set had evens and odds. Odds only set: {:?}", odds_s);
    // task 9..end..

    // task 10 Increment all elements of the set
     let s: HashSet<i32> = HashSet::from([1, 2, 3, 0, -5]);
    
    let result = inc_set(s);
    println!("{:?}", result);
    // task 10..end..

    // task 11 Counting Duplicate Items in a Vector
     let v1 = vec![1, 2, 2, 3, 4, 4, 4, 5, 1];
    println!("Vector {:?} has {} duplicate items.", v1, num_duplicates(v1.clone()));
    
    let v2 = vec![1, 2, 3, 4, 5];
    println!("Vector {:?} has {} duplicate items.", v2, num_duplicates(v2.clone()));

    let v3 = vec![7, 7, 7];
    println!("Vector {:?} has {} duplicate items.", v3, num_duplicates(v3.clone()));
    // task 11..end..
}

// task 1 Size of HashSet
pub fn size_of_set(s: &HashSet<i32>) -> usize {
    s.len()
} 
// task 1..end..

// task 8 Vector Deduplication with HashSet
pub fn dedup(v: Vec<i32>) -> Vec<i32> {
  let v_iter = v.into_iter();
  let s:HashSet<i32> = v_iter.collect();
  let s_iter = s.into_iter();
  let z:Vec<i32> = s_iter.collect();
  z
}
// task 8..end..

// task 9 Filtering HashSet Elements
pub fn remove_evens(s: HashSet<i32>) -> HashSet<i32> {
  let mut h = HashSet::new();
  let s_iter = s.into_iter();
  for i in s_iter{
    if i % 2 != 0 {
        h.insert(i);
    }
  }h
}
// task 9..end..

// task 10 Increment all elements of the set
pub fn inc_set(s: HashSet<i32>) -> HashSet<i32> {
    let mut result = HashSet::new();
let s_iter = s.into_iter();
for i in s_iter{
    result.insert(i+1);
}result

} 
// task 10..end..

// task 11 Counting Duplicate Items in a Vector
pub fn num_duplicates(v: Vec<i32>) -> usize {
    let original_len = v.len();
    let s:HashSet<i32> = v.clone().into_iter().collect();
      original_len - s.len()
} 
// task 11..end..