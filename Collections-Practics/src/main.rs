// task 1 Extending Collection
use std::collections::HashSet;

fn main() {
    // task 1 Extending Collection
    let mut s1 = HashSet::from([1,2,3]);
    let s2 = HashSet::from([3,4,5]);
    
    let mut v1 = vec![1,2,3];
    let v2 = vec![4,5,6];
    
    println!("s1 before: {:?}", s1);
    s1.extend(s2);
    println!("s1 after extend: {:?}", s1);

    println!("v1 before: {:?}", v1);
    v1.extend(v2);
    println!("v1 after extend: {:?}", v1);
    // task 1..end..

    // task 2
    let v = vec![HashSet::from([1,2,3]), HashSet::from([4,5]), HashSet::from([7])];
    let result = largest_set(&v);
    println!("{:?} -> Largest set size: {}", v, result);

    let v_empty: Vec<HashSet<i32>> = vec![];
    let result_empty = largest_set(&v_empty);
    println!("{:?} -> Largest set size: {}", v_empty, result_empty);

    let v_single = vec![HashSet::from([10, 20])];
    let result_single = largest_set(&v_single);
    println!("{:?} -> Largest set size: {}", v_single, result_single);
    // task 2..end..

    // task 3
    let n = 4;
    let result = mul_table(n);
    println!("{:?}", result);
    // task 3..end..
    
    // task 4
     let v1 = vec![HashSet::from([1,2,3,4]), HashSet::from([1,2]), HashSet::from([5,6,7])]; 
    let k1 = 3;
    let result1 = remove_smaller_than_k(&v1, k1);
    println!("Original: {:?}, k={}, Result: {:?}", v1, k1, result1);

    let v2 = vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([3])];
    let k2 = 2;
    let result2 = remove_smaller_than_k(&v2, k2);
    println!("Original: {:?}, k={}, Result: {:?}", v2, k2, result2);
    // task 4..end..

    // task 5
     let v1 = vec![
        HashSet::from([1, 2, 3]), 
        HashSet::from([3, 4, 5]), 
        HashSet::from([5, 6, 7])
    ];
    let result1 = merge_all(&v1);
    println!("Merging {:?} -> {:?}", v1, result1);

    let v2: Vec<HashSet<i32>> = vec![];
    let result2 = merge_all(&v2);
    println!("Merging {:?} -> {:?}", v2, result2);

    let v3 = vec![HashSet::from([10, 20])];
    let result3 = merge_all(&v3);
    println!("Merging {:?} -> {:?}", v3, result3);
    // task 5..end..

    // task 6
        let v1 = vec![
        HashSet::from([1]), 
        HashSet::from([2, 3]), 
        HashSet::from([4, 5, 6])
    ];
    println!("{:?}, find k=2 -> {}", v1, find_set_of_size_k(&v1, 2));
    println!("{:?}, find k=3 -> {}", v1, find_set_of_size_k(&v1, 3));
    println!("{:?}, find k=4 -> {}", v1, find_set_of_size_k(&v1, 4));

    let v2: Vec<HashSet<i32>> = vec![];
    println!("{:?}, find k=1 -> {}", v2, find_set_of_size_k(&v2, 1));
    // task 6..end..
} 

// task 2 Find the Largest Set
pub fn largest_set(v: &Vec<HashSet<i32>>) -> usize {
    if v.is_empty() {
        return 0;
    }

    let mut largest = v[0].len();

    for i in 1..v.len(){
        let current_size = v[i].len();
        if current_size > largest {
            largest = current_size;
        }
    }
    largest
} 
// task 2..end..

// task 3 Multiplication Table
pub fn mul_table(n: u32) -> Vec<Vec<u32>> {
    if n==0{
        return Vec::new()
    }
    let mut table = Vec::new();
    for i in 1..(n+1){
        let mut row = Vec::new();
        for j in 1..(n+1){
            row.push(i*j);
        }
        table.push(row);
    }
    table
} 
// task 3..end..

// task 4 Filter Sets by Size
pub fn remove_smaller_than_k(v: &Vec<HashSet<i32>>, k: usize) -> Vec<HashSet<i32>> {
    let mut result = vec![];
    for i in 0..v.len() {
        if v[i].len() >= k{
            result.push(v[i].clone());
        }
    }result
} 
// task 4..end..

// task 5 Merge All Sets
pub fn merge_all(v: &Vec<HashSet<i32>>) -> HashSet<i32> {
    let mut s = HashSet::new();
    for i in  0..v.len(){
        s.extend(v[i].clone());
    }
    s
} 
// task 5..end..

// task 6 Find Set of Size K
pub fn find_set_of_size_k(v: &Vec<HashSet<i32>>, k: usize) -> i32 {
    for i in 0..v.len(){
        if v[i].len() == k {
            return i as i32;
        }
    }-1
} 
// task 6..end..