use std::collections::{HashMap, HashSet};
// task 1 Automatic Dereferencing 1
#[derive(Debug)]
pub struct S {
    pub z: u32,
}
// task 1..end..

// task 2 Automatic Dereferencing 2
pub struct Pair {
    pub a: i32,
    pub b: i32,
}
// task 2..end..

// task 3 Exercise: Circle Diameter
pub struct Circle {
    pub radius: f32,
}
// task 3..end..

// task 4 Exercise: Push Sum in Struct
#[derive(Debug)]
pub struct V {
    pub vector: Vec<i32>,
}
// task 4..end..

// task 5 Exercise: Increment Point
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
// task 5..end..

// task 6 Downcast Pair
pub struct Pair {
    pub a: u32,
    pub b: u32,
}

#[derive(Debug)]
pub struct SmallPair {
    pub a: u16,
    pub b: u16,
}
// task 6..end..

// task 7 Exercise: Sum of Everything
pub struct Bag {
    pub v: Vec<i32>,
    pub set: HashSet<i32>,
    pub map: HashMap<i32, i32>,
}
// task 7..end..

fn main() {
    // task 1 Automatic Dereferencing 1
    let mut s = S { z: 7 };
    increment(&mut s);
    println!("{:?}", s);
    // task 1..end..

    // task 2 Automatic Dereferencing 2
     let p = Pair { a: 3, b: -1 };

    let result = sum_pair(&p);
    println!("{}", result);
    // task 2..end..

    // task 3 Exercise: Circle Diameter
     let c = Circle { radius: 3.5 };
    let result: f32 = circumference(&c);
    println!("{}", result);
    // task 3..end..

    // task 4 Exercise: Push Sum in Struct
     let mut stv = V {
        vector: vec![1, 2, 3],
    };

    push_sum(&mut stv);
    println!("{:?}", stv);
    // task 4..end..

    // task 5 Exercise: Increment Point
     let mut p = Point { x: 3, y: 15 };
    increment(&mut p);
    println!("{:?}", p);
    // task 5..end..

    // task 6 Downcast Pair
    let p = Pair {
        a: 3,
        b: 1_000_000_000,
    };

    let result = downcast(&p);
    println!("{:?}", result);
    // task 6..end..

    // task 7 Exercise: Sum of Everything
    let bag = Bag {
        v: vec![1, 2, 3],
        set: HashSet::from([1, 2, 3]),
        map: HashMap::from([(1, 1), (2, 2), (3, 3)]),
    };
    let result = sum_all(&bag);
    println!("{:?}", result);
    // task 7..end..
}
// task 1 Automatic Dereferencing 1
pub fn increment(s: &mut S) {
    (*s).z = (*s).z + 1;
}
// task 1..end..

// task 2 Automatic Dereferencing 2
pub fn sum_pair(p: &Pair) -> i32 {
    p.a + p.b
}
// task 2..end..

// task 3 Exercise: Circle Diameter
pub fn circumference(c:&Circle)->f32{
    2.0 * c.radius * std::f32::consts::PI
}
// task 3..end..

// task 4 Exercise: Push Sum in Struct
pub fn push_sum(stv: &mut V) {
 let s = stv.vector.iter().sum::<i32>();
 stv.vector.push(s);
}
// task 4..end..

// task 5 Exercise: Increment Point
pub fn increment(p: &mut Point){
    p.x += 1;
    p.y += 1;
}
// task 5..end..

// task 6 Downcast Pair
pub fn downcast(p: &Pair) -> SmallPair {

    let a2 = if let Ok(x) = u16::try_from(p.a) {
        x
    } else {
        u16::MAX
    };

    let b2 = if let Ok(x) = u16::try_from(p.b) {
        x
    } else {
        u16::MAX
    };

    SmallPair { a: a2, b: b2 }
}
// task 6..end..

// task 7 Exercise: Sum of Everything
pub fn sum_all(bag: &Bag) -> i64 {

    let sum_v: i64 = bag.v.iter().map(|&x| i64::from(x)).sum();
    let sum_set: i64 = bag.set.iter().map(|&x| i64::from(x)).sum();
    
    let sum_keys: i64 = bag.map.keys().map(|&x| i64::from(x)).sum();
    
    let sum_vals: i64 = bag.map.values().map(|&x| i64::from(x)).sum();

    sum_v + sum_set + sum_keys + sum_vals
}
// task 7..end..