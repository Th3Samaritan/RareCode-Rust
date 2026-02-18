use std::collections::{HashSet, HashMap};
// task 1&2 Copy & Clone attribute for structs
#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}
// task 1&2..end..

// task 3 contain non-copy types
#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
}
// task 3..end..

// task 4 struct equality
#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    id: u32,
}
// task 4..end..

// task 5
#[derive(Debug, Clone, Copy)]
pub enum ClothType {
    Wool,
    Cotton,
    Kashmir,
    Nylon,
}
#[derive(Debug, Clone, Copy)]
pub struct Fabric {
   pub  cloth_type: ClothType,
   pub  length_meters: u16,
}
// task 5..end..

// task 6
#[derive(Debug,PartialEq, Hash, Eq)]
pub struct Mono {
    a: i32,
}
// task 6..end..

// task 7
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
   pub x: u32,
   pub  y: u32,
}
// task 7..end..

// task 8 Exercise: Pair to Difference
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone )]
pub struct Pair {
   pub a: i32,
   pub b: i32,
}
// task 8..end..

fn main() {
    // task 1&2 Copy & Clone attribute for structs
    let c1 = Coordinate { x: 1, y: 2 };
    let c2 = c1; 
    
    println!("{:?}", c1); 
    println!("{:?}", c2);
    // task 1&2..end..

    // task 3 contain non-copy types
    let user = User {
        name: String::from("Alice"),
        age: 25,
    };
    
    let user2 = user.clone();
    println!("{:?}", user);
    println!("{:?}", user2);
    // task 3..end..

    // task 4 struct equality
    let p1 = Person {
        name: String::from("Bob"),
        id: 100,
    };
    
    let p2 = Person {
        name: String::from("Bob"),
        id: 100,
    };
    
    if p1 == p2 {
        println!("People are equal!");
    }
    // task 4..end..

    // task 5 structs that contain enums
      let fabric_item = Fabric {
        cloth_type: ClothType::Cotton,
        length_meters: 3,
    };
    
    let price = get_price(fabric_item);
    let synthetic = is_synthetic(fabric_item);
    
    println!("{:?} {} {}", fabric_item, price, synthetic);
    // task 5..end..

    // task 6 Set of Structs
     let hs = HashSet::from([
        Mono { a: 1 },
        Mono { a: 2 },
    ]);
    
    println!("{:?}", hs);
    // task 6..end..

    // task 7
    let v = vec![
        Point { x: 1, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 3, y: 3 },
    ];
    
    let result = to_hashset(&v);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Exercise: Pair to Difference
     let v = vec![
        Pair { a: 1, b: 2 },
        Pair { a: 1, b: 2 },
        Pair { a: 3, b: 3 },
    ];
    
    let result = to_difference(&v);
    println!("{:?}", result);
    // task 8..end..
}

// task 5 structs that contain enums
pub fn get_price(fabric: Fabric) -> u32 {
    let rate = match fabric.cloth_type {
        ClothType::Wool => 100,
        ClothType::Cotton => 20,
        ClothType::Kashmir => 600,
        ClothType::Nylon => 8,
    };
    rate * u32::from(fabric.length_meters)
}

pub fn is_synthetic(fabric: Fabric) -> bool {
    match fabric.cloth_type {
        ClothType::Nylon => true,
        _ => false,
    }
}
// task 5..end..

// task 7 Get Unique Points
pub fn to_hashset(v: &[Point]) -> HashSet<Point> {
    v.iter().copied().collect()
}
// task 7..end..

// task 8 Exercise: Pair to Difference
pub fn to_difference(v: &[Pair]) -> HashMap<Pair, i32> {
    v.iter().map(|&x| {
        (x, x.a - x.b)
    }).collect()
}
// task 8..end..