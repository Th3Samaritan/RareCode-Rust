use std::collections::{HashSet, HashMap};
#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    // task 1 Collections Support Equality
    let str1 = "hello";
    let str2 = "hello";
    let str3 = "world";
    println!("str1 == str2: {}", str1 == str2);
    println!("str1 == str3: {}", str1 == str3);
    

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let v3 = vec![3, 2, 1];
    println!("v1 == v2: {}", v1 == v2); 
    println!("v1 == v3: {}", v1 == v3); 
    
    let s1 = HashSet::from([1, 2, 3]);
    let s2 = HashSet::from([3, 1, 2]);
    println!("s1 == s2: {}", s1 == s2); 
    
    let m1 = HashMap::from([(3, 4), (1, 2)]);
    let m2 = HashMap::from([(1, 2), (3, 4)]);
    println!("m1 == m2: {}", m1 == m2); 

    // task 1..end..

    // task 2 Checking if Two Strings are Anagrams
    let s1 = "listen";
    let s2 = "silent";
    println!("{} and {} are anagrams: {}", s1, s2, are_anagrams(s1, s2));
    
    let s3 = "hello";
    let s4 = "world";
    println!("{} and {} are anagrams: {}", s3, s4, are_anagrams(s3, s4));
    // task 2..end..

    // task 3 Check if Two Vectors Have Same Elements (Ignoring Order)
     let v1 = vec![1, 2, 3, 2, 1];
    let v2 = vec![3, 1, 2, 3];
    println!("Same unique elements: {}", same_elements(&v1, &v2));
    
    let v3 = vec![1, 2, 3];
    let v4 = vec![1, 2, 4];
    println!("Same unique elements: {}", same_elements(&v3, &v4));
    // task 3..end..

    // task 4 Enums Don't Support Equality by Default
     let c1 = Color::Red;
    let c2 = Color::Green;
    let c3 = Color::Blue;
    
    println!("Red == Green: {}", c1 == c2);
    println!("Red == Blue: {}", c1 == c3);
    // task 4..end..

    // task 6 Equality of Options
     let a = Some(3);
    let b = Some(3);
    let c = Some(4);
    let d: Option<i32> = None;
    let e: Option<i32> = None;
    
    println!("a == b {}", a == b);
    println!("a == c {}", a == c);
    println!("d == e {}", d == e);
    // task 6..end..

    // task 7 Some context on PartialEq
     let v1 = vec![1.0, 2.0, f64::NAN, 3.0];
    let v2 = vec![1.0, 2.0, f64::NAN, 3.0];
    
    println!("Direct comparison: {}", v1 == v2);
    println!("Smart comparison: {}", float_vecs_match(&v1, &v2));
    // task 7..end..

    // task 8 Equality of References

    let x = &3;
    let y = &3;
    
    let v1 = &vec![1, 2, 3];
    let v2 = &vec![1, 2, 3];
    
    println!("x == y {}", x == y);
    println!("v1 == v2 {}", v1 == v2);
    // task 8..end..
}

// task 2 Checking if Two Strings are Anagrams
pub fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut map1: HashMap<char, usize> = HashMap::new();
    let mut map2: HashMap<char, usize> = HashMap::new();
    
    for c in s1.chars() {
        if map1.get(&c).is_some() {
            let count = map1.get(&c).unwrap();
            map1.insert(c, count + 1);
        } else {
            map1.insert(c, 1);
        }
        }
    
    for c in s2.chars() {
     if map2.get(&c).is_some() {
            let count = map2.get(&c).unwrap();
            map2.insert(c, count + 1);
        } else {
            map2.insert(c, 1);
        }
}
map1 == map2
}
// task 2..end..

// task 3 Check if Two Vectors Have Same Elements (Ignoring Order)
pub fn same_elements(v1: &[i32], v2: &[i32]) -> bool {
    let set1: HashSet<i32> = v1.iter().copied().collect();
    let set2: HashSet<i32> = v2.iter().copied().collect();
    
    set1 == set2
}
// task 3..end..

// task 7 Some context on PartialEq
pub fn float_vecs_match(v1: &Vec<f64>, v2: &Vec<f64>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
	for i in 0..v1.len() {
        let a = v1[i];
        let b = v2[i];
        
        if a.is_nan() && b.is_nan() {
            continue;
        }
        
        if a != b {
            return false;
        }
    }
    
    true
}
// task 7..end..