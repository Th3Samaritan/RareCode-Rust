fn main() {
    // task 1 Display Modified Vector
	let mut v = vec![5,6,7];

	v[0] = 99;
	
	println!("{:?}", v);
    // task 1..end..

    // task 2 Add an Element to a Vector
    let mut v = vec![4, 5, 6];
  v.push(99);
  println!("{:?}", v);
  //  task 2..end..

  // task 3 Generate a Sequence of Integers
  let result = simple_count(6);
  println!("{:?}", result);
    // task 3..end..

    // task 4
    let result = countdown(6);
  println!("{:?}", result);
    // task 4..end..

    // task 5 Functions cannot mutate vectors by default
    let mut v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
    // task 5..end..

    // task 6 Modify Vector with Copying
    let v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
    // task 6..end..

    // task 7 
    let v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
    // task 7..end..

    // task 8
    let v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
    // task 8..end..

    // task 9
    let v = vec![4,2,6];
	let result = add_1_to_each(v);
	println!("{:?}", result);
    // task 9..end..

    // task 10
    let v = vec![4,2,6];
	let result = append_sum(v);
	println!("{:?}", result);
    // task 10..end..

    // task 11 Remove Odd Numbers from Vector
    let v = vec![1,2,3,4];
	let result = remove_odd(v);
	println!("{:?}", result);
    // task 11..end..

    // task 12
    let v = vec![1,2,3,4];
	let k = 3;
	let result = remove_less_than_k(v, k);
	println!("{:?}", result);
    // task 12..end..

    // task 13
    let v1 = vec![1,2,3,4];
	let v2 = vec![0,1,0,4];
	
	let result = elementwise_add(v1, v2);
	println!("{:?}", result);
    // task 13..end..

    // task 14
    let v = vec![1,2,3,4];
	
	let result = reverse(v);
	println!("{:?}", result);
    // task 14..end..
   
    // task 15
    let v = vec![1,2,3,4];
	let idx = 1;
	
	let result = double_at_idx(v, idx);
	println!("{:?}", result);
    // task 15..end..

    // task 16
let v = vec![1,2,3,4];
	let i = 1;
	let j = 2;
	
	let result = swap_ij(v, i, j);
	println!("{:?}", result);
// task 16..end..
} 

// task 3 generate a sequence of integers
pub fn simple_count(n:u32) -> Vec<u32> {
    let mut v = Vec::new();
    for i in 1..n + 1 {
        v.push(i)
} 
v
}
// task 3..end..

// task 4 Generate a Countdown Sequence
pub fn countdown(n: u32) -> Vec<u32> {
    
    let mut v = Vec::new();
    for i in 1..(n+1) {
        v.push(n-i);
    }
    v
}
// task 4..end..

// task 5 Functions cannot mutate vectors by default
pub fn modify(v: Vec<i32>) -> Vec<i32> {
   // v.push(6); 
	v
} 
// task 5..end..

// task 6 Modify Vector with Copying
pub fn modify(v: Vec<i32>) -> Vec<i32> {
  let mut my_vec = vec![];

  for i in 0..v.len() {
	  my_vec.push(v[i]);
  }
  my_vec.push(99);
  my_vec
} 
// task 6..end..

// task 7 Make a Copy of a Vector
pub fn modify(v: Vec<i32>) -> Vec<i32> {
    let my_vec = make_copy(v);
    //my_vec.push(6);
    my_vec
}

pub fn make_copy(v: Vec<i32>) -> Vec<i32> {
    let mut y = vec![];

    for i in 0..v.len() {
        y.push(v[i]);
    }
   y
} 
// task 7..end..

// task 8 Cloning a vector
pub fn modify(v: Vec<i32>) -> Vec<i32> {
    let mut my_vec = v.clone();
    
    my_vec.push(6);
    my_vec
} 
// task 8..end..

// task 9 Add 1 to Each Element in a Vector
pub fn add_1_to_each(v:Vec<i32>) -> Vec<i32> {
    let mut new_vec = v.clone();
    for i in 0..new_vec.len(){
        new_vec[i]+=1
    }
    new_vec
}
// task 9..end..

// task 10 Append Sum of Elements
pub fn append_sum(v: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    for i in 0..v.len(){
        sum = sum + v[i];
    }
    let mut y = v.clone();
    y.push(sum);
    y
} 
// task 10..end..

// task 11 Remove Odd Numbers from Vector
pub fn remove_odd(v:Vec<i32>)-> Vec<i32> {
    let mut y = vec![];
    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            y.push(v[i]);
        }
    }
    y
}
// task 11..end..

// task 12 Remove Elements Less Than K
pub fn remove_less_than_k(v:Vec<i32>, k:i32)->Vec<i32>{
let mut y = vec![];
for i in 0..v.len() {
    if v[i]>=k{
        y.push(v[i]);
    }
}y
}
// task 12..end..

// task 13 Element-wise Addition of Two Vectors
pub fn elementwise_add(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut v3 = vec![];
    for i in 0..v1.len(){
        v3.push( v1[i] + v2[i])
    }
    v3
} 
// task 13..end..

// task 14 Reverse a Vector
pub fn reverse(v: Vec<i32>) -> Vec<i32> {
    let mut y = vec![];
    for i in (0..v.len()).rev(){
        y.push(v[i]);
    }
    y
} 
// task 14..end..

// task 15 Double the Value at a Specific Index
pub fn double_at_idx(v: Vec<i32>, idx: usize) -> Vec<i32> {
    let mut y = v.clone();
    for i in idx..v.len(){
        if v[i] ==v[idx] {
            y[i]=y[i]*2
        } 
    }y
} 
// task 15..end..

// task 16 Swap Elements at Indices i and j
pub fn swap_ij(v: Vec<i32>, i: usize, j: usize) -> Vec<i32> {
    let mut y = v.clone();
        if i<y.len() && j<y.len(){
            let swap = y[i];
            y[i]=y[j];
            y[j]=swap;
        }
        y
} 
// task 16..end..