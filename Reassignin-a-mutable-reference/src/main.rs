use std::collections::HashMap;
use std::collections::HashSet;

// task 1 Mutable reference reassign
fn main() {
    
    let mut v = vec![1,2,3];
    
    let mut_ref = &mut v;
    
    *mut_ref = vec![4,5,6];
    
     println!("{:?}", v);
 // task 1..end..
 // task 2 Exercise: reassign to longest
 let mut v = vec![];
      let u = vec![1,2,3];
    let w = vec![1,2,3,4];
    
    assign_to_longest(&mut v, u, w);
    println!("{:?}", v);
// task 2..end..

// task 3 Exercise: Remove Evens Via Mutable Ref
     remove_even(&mut v);
    println!("{:?}", v)


}


pub fn assign_to_longest(v: &mut Vec<i32>, u: Vec<i32>, w: Vec<i32>) {
    *v = if u.len() >= w.len() {
        u
    } else {
        w
    };
}
// task 2..end..

// task 3 Example: Remove Evens Via Mutable Ref
pub fn remove_even(v: &mut Vec<i32>) {
    
    let mut new_v = vec![];
    for e in v.iter() {
        if *e % 2 == 1 {
            new_v.push(*e);
        }
    }
    *v = new_v; 
}

// task 3..end..

// task 4 Exercise: Reassign to the vector with the largest max
fn main() {
    let mut v: Vec<i32> = vec![];
    
    let a = vec![1,2,3];
    let b = vec![1,2,3,4];
    
    assign_to_vector_with_greatest_max(&mut v, a, b);
    println!("{:?}", v);
}

pub fn assign_to_vector_with_greatest_max(v: &mut Vec<i32>, a: Vec<i32>, b: Vec<i32>) {
    let max_a = a.iter().max();
    let max_b = b.iter().max();
    if max_a.is_none() && max_b.is_none(){
        *v=a;
    } else if max_a.is_none() {
        *v=b;
    } else if max_b.is_none(){
        *v=a;
    } else if max_a.unwrap() >= max_b.unwrap() {
        *v=a;
    } else {
        *v=b;
    }
}

// task 4..end..

// task 5 Exercise: Reassign set to keys of hashmap

fn main() {
    let hm = HashMap::from([(1,10), (2, 20), (3, 30)]);
    
    let mut s = HashSet::new();
    
    assign_keys(&mut s, hm);
    
    println!("{:?}", s);
}

pub fn assign_keys(s: &mut HashSet<i32>, hm: HashMap<i32, i32>) {
    *s =  hm.keys().copied().collect();
 // task 5..end..

 // task 6 Exercise: Assign Vector of Double References to Set
 
fn main() {
    
    let v = vec![&&1, &&2, &&3];
    let mut s: HashSet<i32> = HashSet::new();
    
    assign(&mut s, v);
    
    println!("{:?}", s);
}

pub fn assign(s: &mut HashSet<i32>, v: Vec<&&i32>) {
    *s = v.into_iter().copied().copied().collect();
}
// task 6..end..

// task 7 Assigning takes ownership
fn main() {
    
    let mut v = vec![1,2,3];
    let w = vec![7,8,9];
    
    let r = &mut v;
    
    *r = w;
    //println!("{:?}", w);
    println!("{:?}", v);
}
// task 7..end..

// task 8 Exercise: Reassign to the vector with the largest sum in a nested vector

fn main() {
    let v_big = vec![vec![1,2,-5], vec![2,-20,3], vec![1,-1,-1], vec![0, 2, -4]];
    
    let mut t: Vec<i32> = vec![];
    
    assign_to_largest_sum_in_nested(&mut t, v_big);
    
    println!("{:?}", t);
}

pub fn assign_to_largest_sum_in_nested(v: &mut Vec<i32>, v_big: Vec<Vec<i32>>) {
    
    let mut c_max = i32::MIN;
    for i in v_big {
        let s: i32 = i.iter().sum();

        if s >= c_max {
            c_max = s;
            *v = i;
        }
    }
}