use std::collections::{HashSet, HashMap};

fn main() {
    // task 1 Introduction to .into()
    let s: HashSet<i32> =[1,2,3].into();
    println!("{:?}", s);
    // task 1..end..

    // task 2 Array of Tuples .into() HashMap
      let a = [(1, 7), (2, 11), (3, 13)];
    
    let m: HashMap<i32, i32> = a.into();
    println!("{:?}", m);
    // task 2..end..

    // task 3 Exercise: Convert integers to larger ones
     let v: Vec<i32> = [1, 2, 3].into();
    let result = upconvert(v);
    println!("{:?}", result);
    // task 3..end..

    // task 4 Slice into vector
     let a = [1, 2, 3];
    let result = rest_to_vec(&a);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Consumption with ::from and .into
      let a = [1, 2, 3];
    
    let v: Vec<i32> = a.into();
    println!("{:?}", v);
    // is `a` consumed?

    let b = [vec![1], vec![2], vec![3]];
    
    let w: Vec<Vec<i32>> = b.into();
    println!("{:?}", w);
    // is `b` consumed?
    
    // ans: b is consumed, a is not because it implements Copy
    // task 5..end..
}

// task 3 Convert integers to larger ones
pub fn upconvert(v: Vec<i32>) -> Vec<i64> {
    v.into_iter().map(|x| x.into()).collect()
}
// task 3..end..

// task 4 Slice into vector
pub fn rest_to_vec(s: &[i32]) -> Vec<i32> {
    if s.is_empty(){
       return vec![];
    }
    s[1..].into()
}
// task 4..end..