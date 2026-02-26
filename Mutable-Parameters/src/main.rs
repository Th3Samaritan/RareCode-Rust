use std::collections::{HashSet, HashMap};
fn main() {
    // task 1 mutable parameter
	let v = vec![1,2,3];
	let res = append_zero(v);
	println!("{:?}", res);
    // task 1..end..

    // task 2 Consumption of a mutable parameter
     let v = vec![1, 2, 3];
    let res = append_zero(v);
    println!("{:?}", res);
  //  println!("{:?}", v); // comment this out so the code compiles
  // task 2..end..

  // task 3
   let v = vec![1,2,3];
    let result = append_sum(v);
    println!("{:?}", result);
    // task 3..end..

    // task 4 Remove Zero
    let mut set = HashSet::new();

	set.insert(0);
    set.insert(1);
    set.insert(2);

    let result = remove_zero(set);
    println!("{:?}", result);
	
    // task 4..end..

    // task 5 Exercise: Mutable parameter 1
    let v = vec![1,2,3];
	let idx = 1;
	let result = double_at(v, idx);
	println!("{:?}", result);
    // task 5..end..

    // task 6
    let v = vec![1,2,3,4,5,6];
	let result = reverse_in_place(v);
	println!("{:?}", result);
    // task 6..end..

    // task 7 Remove from Set
     let v = vec![1,2];
    let s = HashSet::from([1,2,3,4]);
    let result = remove_from_set(&v, s);
    println!("{:?}", result); // {3,4} or {4,3}
    // task 7..end..

    // task 8 Inc Odd
     let v = vec![1,2,3];
    let result = inc_odd(v);
    println!("{:?}", result);
    // task 8..end..

    // task 9 Add Max to All
    let v1 = vec![1,2,3];
    let v2 = vec![4,5,6];
    
    let result = add_max_to_all(&v1, v2);
    println!("{:?}", result);
    // task 9..end..

    // task 10 Calculate Absolute Values
    let v = vec![1, -2, 3, -4];
	
	let result = absolute_value(&v);
	println!("{:?}", result);
    // task 10..end..

    // task 11
     let v1 = vec![1,2,3];
    let v2 = vec![4,5,6];

    let hm = HashMap::new();

    let result = augment(hm, &v1, &v2);

    println!("{:?}", result);
    // task 11..end..

    // task 12
    let v: Vec<i32> = vec![1, 2, 3];
	
	let result = append_sum(&v);
	println!("{:?}", result);
    // task 12..end..

    // task 13 Merge Key-value Pairs
    let a: HashMap<i32, i32> = HashMap::from([(1,10), (2, 20)]);
    let b: HashMap<i32, i32> = HashMap::from([(2,4), (3, 9)]);
    
    let result = merge(a, &b);
    println!("{:?}", result);
    // task 13..end..
}

// task 1 mutable parameter
pub fn append_zero(mut v: Vec<i32>) -> Vec<i32> {
	v.push(0);
	v
}
// task 1..end..

// task 2 Consumption of a mutable parameter
pub fn append_zero(mut v: Vec<i32>) -> Vec<i32> {
    v.push(0);
    v
}

// task 2..end..

// task 3 Params on Mut
pub fn append_sum(mut v: Vec<i32>) -> Vec<i32> {
    let sum: i32 = v.iter().sum();
    v.push(sum);
    v
}
// task 3..end..

// task 4 Remove Zero
pub fn remove_zero(mut s: HashSet<i32>) -> HashSet<i32> {
    s.remove(&0);
    s
}
// task 4..end..

// task 5 Exercise: Mutable parameter 1
pub fn double_at(mut v: Vec<i32>, idx: usize)->Vec<i32>{
    if idx < v.len() {
        v[idx] = v[idx] * 2;
    }
    v
}
// task 5..end..

// task 6 Reverse in Place
pub fn reverse_in_place(mut v: Vec<i32>) -> Vec<i32> {
    let n = v.len();
    // your co
    for i in 0..n/2{
       let m =  n-i-1;

       let temp = v[i];
       v[i] = v[m];
       v[m] = temp;

    }
    v
}
// task 6..end..

// task 7 Remove from Set
pub fn remove_from_set(v: &Vec<i32>, mut s: HashSet<i32>) -> HashSet<i32> {
    // your code here
    for i in 0..v.len(){
        let element = &v[i];
        s.remove(element);
    }s
}
// task 7..end..

// task 8 Inc Odd

pub fn inc_odd(mut v: Vec<u32>) -> Vec<u32> {
    // your code here
    for i in 0..v.len(){
        if v[i] % 2 != 0 {
            v[i] +=1;
        }
    }v
}
// task 8..end..

// task 9 Add Max to All
pub fn add_max_to_all(first: &Vec<i32>, mut second: Vec<i32>) -> Vec<i32> {
    // your code here
    let max_val = first.iter().max().copied().unwrap_or(0);
   
    for i in 0..second.len() {
    second[i] += max_val;
} 
second
}
// task 9..end..

// task 10 Calculate Absolute Values

pub fn absolute_value(v: &Vec<i32>) -> Vec<i32> {
	let mut my_v = vec![];
    for i in 0..v.len(){
       let my =  if v[i] < 0 {
      -1 * v[i]
        } else {
            v[i]
        };
        my_v.push(my);
    } 
	// TODO
	
	my_v
} 
// task 10..end..

// task 11 Mutable HashMap
pub fn augment(mut hm: HashMap<i32, i32>, v1: &Vec<i32>, v2: &Vec<i32>) -> HashMap<i32, i32> {
    // your code here
    for i in 0..v1.len() {
        let key = v1[i];
        let value = v2[i];
        hm.insert(key, value);
    }
    hm
}
// task 11..end..

// task 12 Append Sum
pub fn append_sum (v: &Vec<i32>)->Vec<i32> {
    let mut sum = v.clone();
    let sum_all = v.into_iter().sum();
    sum.push(sum_all);
    sum
}
// task 12..end..

// task 13 Merge Key-value Pairs
pub fn merge(mut a: HashMap<i32, i32>, b: &HashMap<i32, i32>) -> HashMap<i32, i32> {
    // your code her
    for (k, v) in b{
        if !a.contains_key(k) {
            a.insert(*k, *v);
    }
}   
a
}
// task 13..end..