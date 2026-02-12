use std::collections::{HashMap, HashSet};
fn main() {
    // task 1 Iterating on tuples
    let a = [(1, 2), (3, 4), (5, 6)];
    let result = sum_tuples(&a);
    println!("{:?}", result);
        // task 1..end..
    
    // task 2 Example: Using Map to Discard
    let a = [(1, 2, 3), (4, 5, 6), (7, 8, 9)];
    let result = vec_from_middle(a);
    println!("{:?}", result);
    // task 2..end..

    // task 3 Map and filter on a HashMap
     let map: HashMap<i32, i32> = HashMap::from([(1, 2), (3, 4)]);
    let result = sum_key_values_pairs(map);
    println!("{}", result);
    // task 3..end..

    // task 4 Exercise: Filter tuples
     let v = vec![(1, 5), (3, 6), (2, 6), (3, 7), (4, 8)];
    let k = 9;
    let result = filter_tuples(v, k);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Exercise: Add value to index
    let a = [1, 0, 4];
    let result = add_to_index(&a);
    println!("{:?}", result);
    // task 5..end..

    // task 6 Exercise: Remove if index is in set
     let a = [1, 0, 4];
    let set: HashSet<usize> = HashSet::from([1, 2]);

    let result = remove_if_idx_in_set(&a, &set);
    println!("{:?}", result);
    // task 6..end..

    // task 7 Example: Destructure Nested Tuple
     let a = [(1, (2, 3)), (4, (5, 6))];

    let result = a.iter().map(|(x, (y, z))| *x + *y + *z).collect::<Vec<i32>>();
    println!("{:?}", result);
// task 7..end..

// task 8 Exercise: Sum Nested Tuple
let a = [((1, 2), (3, 4)), ((5, 6), (7, 8))];
    let result = sum_nested_tuples(a);
    println!("{:?}", result);
// task 8..end..

// task 9 Exercise: String and enumerate 1
let s = "hello, world!".into();

    let result = replace_at(s, 1, '3');
    println!("{}", result);
    // task 9..end..

// task 10 Exercise: String and enumerate 2
 let s = "hello, world!".into();

    let result = remove_at(s, 1);
    println!("{}", result);
    // task 10..end..
}
// task 1 Iterating on tuples
pub fn sum_tuples(a: &[(i32, i32)]) -> Vec<i32> {
    a.into_iter().map(|&(x,y)| x + y).collect()
}
// task 1..end..

// task 2 Example: Using Map to Discard
pub fn vec_from_middle(a: [(i32, i32, i32); 3]) -> Vec<i32> {
    a.into_iter().map(|(_,x,_)| x).collect()
}
// task 2..end..

// task 3 Map and filter on a HashMap

pub fn sum_key_values_pairs(map: HashMap<i32, i32>) -> i32 {
    map.iter().map(|(&x, &y)| x + y).sum()
}

// task 3..end..

// task 4 Exercise: Filter tuples
pub fn filter_tuples(v: Vec<(i32, i32)>, k: i32) -> Vec<(i32, i32)> {
    v.into_iter().filter(|&(x, y)| x + y >=k).collect::<Vec<(i32, i32)>>()
}
// task 4..end..

// task 5 Exercise: Add value to index
pub fn add_to_index(sl: &[i32]) -> Vec<i32> {
    sl.iter().enumerate().map(|(i, &x)| x+ i as i32).collect()
}
// task 5..end..

// task 6 Exercise: Remove if index is in set
pub fn remove_if_idx_in_set(arr: &[i32], set: &HashSet<usize>) -> Vec<i32> {
    arr.iter().enumerate().filter(|(i, _x)| !set.contains(i)).map(|(_, &x)| x).collect()
}
// task 6..end..

// task 8: Exercise: Sum Nested Tuple
pub fn sum_nested_tuples(arr: [((i32, i32), (i32, i32)); 2]) -> Vec<(i32, i32)> {
    arr.iter().map(|((w,x), (y,z))| (w + x , y + z)).collect()
}
// task 8..end..

// task 9 Exercise: String and enumerate 1
pub fn replace_at(s: String, index: usize, c: char) -> String {
    let s1: String = s.chars().enumerate().map(|(i, ch)| {if  i == index {c} else {ch}}).collect();
    s1
}
// task 9..end..

// task 10 Exercise: String and enumerate 2

pub fn remove_at(s: String, index: usize) -> String {
     s.chars().enumerate().filter(|(i, _)| *i != index).map(|(_, c)| c).collect()
}

// task 10..end..