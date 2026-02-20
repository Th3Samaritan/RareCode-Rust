
fn main() {
    // task 1 Vector Creation and Display
	let my_vec = vec![1,2,3];
    
	println!("{:?}", my_vec);
    // task 1..end..

    // task 2 Vector Length
    let my_vec = vec![1,3];

	let length = my_vec.len();
	println!("{}", length);
    // task 2..end..

    // task 3 Vector Iteration and Printing
    let my_vec = vec![1,2,3];
	
	for i in 0..my_vec.len() {
		let element = my_vec[i];
		println!("{}", element);
	}
    // task 3..end..

    // task 4 Maximum Value in Vector
    let my_vec = vec![1,2,3];
	let result = max(my_vec);
	
	println!("{}", result);
    // task 4..end..

    // task 5 All Greater Than k
    let my_vec = vec![1,2,3];
	let k = 0;
	let result = greater_than_k(my_vec, k);
	
	println!("{}", result);
    // task 5..end..

    // task 6 Find Value K in Vector
     let k = 8;
    let v = vec![1, 2, 8, 9, 3];    
    let result = find_k(v, k);
    println!("{}", result);
    // task 6..end..

    // task 7
     let index_at = 2;
    let v = vec![2, 4, 6, 8];    
    let result = get_index_at(v, index_at);
    println!("{}", result);
    // task 7..end..

    // task 8 Compare Elements at Indices
    let i = 0;
	let j = 1;
	let v = vec![5,3,15,21];
	
	let result = index_greater(v, i, j);
	println!("{}", result);
    // task 8..end..

    // task 9 Check if Vector is Sorted
   let v = vec![1,2,3];
	
	let result = is_sorted(v);
	println!("{}", result);
    // task 9..end..

    // task 10
    let v = vec![1,2,1,3,4];
	let result = first_unsorted(v);
	println!("{}", result);
    // task 10..end..

    // task 11 Palindrome Vector
    let v = vec![1,2,3,2,1];
	
	let result = is_palindrome(v);
	println!("{}", result);
    // task 11..end..

    // task 12 Count Odd Numbers in Vector
    let v = vec![1,2,3,4,5];
	
	let result = count_odds(v);
	println!("{}", result);
    // task 12..end..

    // task 13 At Least K Elements Larger Than T
    let v = vec![1,5,8,12,3];
	let k = 2;
	let t = 4;
	
	let result = at_least_k_larger_than_t(v, k, t);
	println!("{}", result);
    // task 13..end..

    // task 14
    let v = vec![1,2,3,4,5];
	let k = 3;
	let idx = 4;
	
	let result = k_appears_before_idx(v, k, idx);
	println!("{}", result);
    // task 14..end..

    // task 15
    let v = vec![1,2,1,4,5];
	let k = 1;
	
	let result = contains_k_twice(v, k);
	println!("{}", result);
    // task 15..end..
} 

// task 4 Maximum Value in Vector
pub fn max(v: Vec<u32>) -> u32 {
  let mut biggest = 0;
  for i in 0..v.len() {
    if v[i] > biggest {
      biggest = v[i];
    }
  }
  
  biggest
} 
// task 4..end..

// task 5 All Greater Than k
pub fn greater_than_k(v: Vec<u32>, k: u32) -> bool {
  for i in 0..v.len(){
    if v[i] <= k {
        return false
    }
  }
  true
} 
// task 5..end..

// task 6 Find Value K in Vector
pub fn find_k(v: Vec<i32>, k: i32) -> usize {
    for i in 0..v.len() {
        if v[i] == k {
            return i
        }
    }
    0
}
// task 6..end..

// task 7 Get Element at Index
pub fn get_index_at(v: Vec<i32>, i: usize) -> i32 {
    v[i]
}
// task 7..end..

// task 8 Compare Elements at Indices
pub fn index_greater(v: Vec<u32>, i: usize, j: usize) -> bool {
        if v[i] >= v[j] {
            return true;
        }
        else {
            return false
        }
}
// task 8..end..

// task 9 Check if Vector is Sorted
pub fn is_sorted(v: Vec<i32>) -> bool {
    for i in 0..v.len()-1{
        if v[i] > v[i+1] {
            return false;
        }
    }
    return true
} 
// task 9..end..

// task 10 First Unsorted Element
pub fn first_unsorted(v: Vec<i32>) -> usize {
    if v.len() < 2 {
        return 0;
    }
    for i in 1..v.len() {
        if v[i] < v[i-1]{
            return i;
        }
    }
    return 0
}
// task 10..end..

// task 11 Palindrome Vector
pub fn is_palindrome(v: Vec<i32>) -> bool {
   
   if v.len() <= 1{
    return true;
   }
    for i in 0..v.len()/2{
        if v[i] == v[v.len() - 1 - i] {
          return true;
        }
    }
    false
} 
// task 11..end..

// task 12 Count Odd Numbers in Vector

pub fn count_odds(v: Vec<i32>) -> i32 {
let mut counts = 0;
    for i in 0..v.len() {
        if v[i] % 2!=0 {
            
            counts = counts + 1
        }
    }
    counts    
} 
// task 12..end..

// task 13 At Least K Elements Larger Than T
pub fn at_least_k_larger_than_t(v: Vec<i32>, k: usize, t: i32) -> bool {
    let mut count = 0;
    for i in 0..v.len() {
    
        if t < v[i] {
            count = count + 1
        } 
        }
        if count < k {
            return false;
        }
    true
} 
// task 13..end..

// task 14 K Appears Before Index
pub fn k_appears_before_idx(v: Vec<i32>, k: i32, idx: usize) -> bool {
    for i in 0..idx {
      if v[i] == k {
        return true;
      }
    } 
    false
}
// task 14..end..

// task 15 Contains K Twice

pub fn contains_k_twice(v: Vec<i32>, k: i32) -> bool {
    let mut count = 0;
    for i in 0..v.len(){
        if v[i] == k{
            count = count + 1;
        }
    }
    if count >= 2 {
        return true;
    }
    
    false
} 
// task 15..end..