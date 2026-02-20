// task 1 Trait bounds
pub fn first_two<T: Copy>(v: Vec<T>) -> Option<(T, T)>{
    if v.len() < 2 {
        return None;
    }

    Some((v[0], v[1]))
}
// task 1..end..

// task 2 Exercise: Fix Compilation with Traits
#[derive(Debug, Clone)]
pub enum E {
    A,
    B,
    C,
}

pub fn first_two<T: Clone>(v: Vec<T>) -> Option<(T, T)> {
    if v.len() < 2 {
        return None;
    }

    Some((v[0].clone(), v[1].clone()))
}
// task 2..end..

// task 3
use std::fmt::Debug;
#[derive(Debug)]
pub enum E {
    A,
    B,
    C,
}

#[derive(Debug)]
pub struct S {
   pub field: u32,
}

pub fn print_it<T: std::fmt::Debug>(a: T) {
    println!("{:?}", a);
}
// task 3..end..

// task 4 Clone Trait Bound
pub fn clone_elements<T>(s: &Vec<T>) -> Vec<T> where T: Clone{
    
    s.into_iter().map(|x| x.clone()).collect()
}
// task 4..end..

// task 5 Exercise: Fix Compilation Issue
pub fn first_two<T: Clone>(s: &[T]) -> Option<(T, T)> {
    if s.len() < 2 {
        return None
    }
    
    Some((s[0].clone(), s[1].clone()))
}
// task 5..end..

// task 6
#[derive(Debug, PartialEq)]
pub enum E {
    A,
    B,
    C,
}

pub fn equal_at<T: PartialEq>(v: Vec<T>, i: usize, j: usize) -> bool {
    v[i] == v[j]
}

// task 6..end..

// task 7
#[derive(Debug, PartialEq, Eq)]
pub enum E {
    A,
    B,
    C,
}

pub fn equal_at<T: PartialEq>(v: Vec<T>, i: usize, j: usize) -> bool {
    v[i] == v[j]
}

// task 7..end..

// task 8 Match Last
pub fn find_last<T: PartialEq>(v: Vec<T>, k:T)->Option<usize>{
    for (i, e) in v.into_iter().enumerate().rev() {
        if e == k {
            return Some(i);
        }
    }
    None
}
// task 8..end..

// task 9 Exercise: Find Last on a Slice
pub fn find_last<T: PartialEq>(v: &[T], k:T)-> Option<usize>{
    for (i, e) in v.iter().enumerate().rev() {
        if *e == k {
            return Some(i);
        }
    }
    None
}
// task 9..end..

// task 10 Exercise: First Is Last
#[derive(Eq, PartialEq)]
pub enum E {
    A,
    B,
    C,
}

pub fn first_is_last<T: PartialEq>(v: &[T]) -> bool {
    if v.len() == 0 {
        return false;
    }
    
    v[0] == v[v.len() - 1]
}

// task 10..end..

fn main() {
    // task 1 Trait bounds
    let v = vec![1,2,3,4];
    let result = first_two(v);
    println!("{:?}", result);

    let v = vec![5.3];
    let result = first_two(v);
    println!("{:?}", result);

    let v: Vec<i32> = vec![];
    let result = first_two(v);
    println!("{:?}", result);
    // task 1..end..

    // task 2 Exercise: Fix Compilation with Traits
    let v = vec![E::A, E::C, E::C];
    let result = first_two(v);
    println!("{:?}", result);

    let v: Vec<i32> = vec![];
    let result = first_two(v);
    println!("{:?}", result);
    // task 2..end..

    // task 3 Debug Trait
     let v = vec![E::A, E::C, E::C, E::B];
    print_it(v);
    
    let v = vec![S { field: 3 }];
    print_it(v);

    let a = 3;
    print_it(a);
    // task 3..end..

    // task 4 Clone Trait Bound
     
    let v = vec!["hello", "world"];
    let result = clone_elements(&v);
    println!("{:?}", result);
    
    let v = vec![1, 2];
    let result = clone_elements(&v);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Exercise: Fix Compilation Issue
    let v1 = vec![1, 2, 3];
    let result = first_two(&v1);
    println!("{:?}", result);

    let v2 = [vec![1], vec![2]];
    let result = first_two(&v2);
    println!("{:?}", result);
    // task 5..end..

    // task 6 PartialEq Trait Bound
     let v = vec![E::A, E::C, E::C, E::B];
    let result = equal_at(v, 1, 2);
    println!("{:?}", result);
    // task 6..end..

    // task 7 PartialEq is a supertrait of Eq
    let v = vec![E::A, E::C, E::C, E::B];
    let result = equal_at(v, 1, 2);
    println!("{:?}", result);
    
    let v = vec![1.0, 1.0];
    let result = equal_at(v, 0, 1);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Match Last
     let v = vec![ "world".to_string(), "hello".to_string(), "world".to_string(), "RareCode".to_string()];
    let result = find_last(v, "world".to_string());
    println!("{:?}", result);
    // task 8..end..

    // task 9 Exercise: Find Last on a Slice
       let v = vec![ "world".to_string(), "hello".to_string(), "world".to_string(), "RareCode".to_string()];
    let result = find_last(&v, "world".to_string());
    println!("{:?}", result);
    
    let v = [1,2,3,2];
    let result = find_last(&v, 3);
    println!("{:?}", result);
    // task 9..end..

    // task 10 Exercise: First Is Last
     let v = vec![E::A, E::C, E::C, E::B];
    let result = first_is_last(&v);
    println!("{:?}", result); 

    let v = vec![E::A, E::C, E::C, E::A];
    let result = first_is_last(&v);
    println!("{:?}", result); 
    
    let v = vec!["a".to_string(), "b".to_string(), "a".to_string()];
    let result = first_is_last(&v);
    println!("{:?}", result);
    // task 10..end..
}