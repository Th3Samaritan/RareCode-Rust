use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
// task 1 Eq Function
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
// task 1..end..

// task 2
#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}
// task 2..end..

// task 4 The clone function
#[derive(Debug, PartialEq, Clone)]
pub enum RedBlack {
    Red,
    Black,
}
// task 4..end..

// task 5
#[derive(PartialEq)]
pub enum RedBlack {
    Red,
    Black,
}
// task 5..end..

// task 6
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
// task 6..end..

fn main() {
    // task 1 Eq Function
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    
    let result = p1.eq(&p2);

    println!("{}", result);
    // task 1..end..

    //  task 2 ge function
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 2, y: 3 };
    
    let result = p1.ge(&p2);

    println!("{}", result);
    // task 2..end..

    // task 3 Exercise: a < b
     let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 2, y: 3 };
    
    let result = p1.le(&p2);

    println!("{}", result);
    // task 3..end..

    // task 4 The clone function
    let c1 = RedBlack::Red;
    let c2 = c1.clone();
    let result = c1 == c2;
    
    println!("{}", result);
    // task 4..end..

    // task 5 Exercise: Not Equal
    let c1 = RedBlack::Red;
    let c2 = RedBlack::Black;
    let result = c1.ne(&c2);
    
    println!("{}", result);
    // task 5..end..

    // task 6 Hash Trait
    let p1 = Point { x: 2, y: 3 };
    
    let mut hasher = DefaultHasher::new();
    p1.hash(&mut hasher);
    let result = hasher.finish();
    
    println!("{:?}", result);
    // task 6..end..
}