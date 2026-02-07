use std::collections::HashSet;
use std::collections::HashMap;
fn main() {
    // task 1 HashSet::from
    let arr = HashSet::from([1, 2, 3]);
    println!("{:?}", arr);
    // task 1..end..

    // task 2 Vec::from
      let a = [1, 2, 3];
    let v: Vec<i32> = Vec::from(a);
    println!("{:?}", v);
    // task 2..end..

    // task 3 HashMap from Pairs of Tuples
     let pairs = [(1, 10), (2, 20), (3, 30)];
    let _m: HashMap<i32, i32> = HashMap::from(pairs);
    // task 3..end..

    // task 4 Vec from slice
     let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    let result = Vec::from(slice);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Exercise: Assign Vector with Greatest Max Value
     let v: Vec<u8> = vec![1, 2, 3, 4, 5];

    let result = bigger(v);
    println!("{:?}", result)
    // task 5..end..

    // task 6 From bool
    let b = true;
    let result = zero_one(b);
    println!("{}", result);

    let b = false;
    let result = zero_one(b);
    println!("{}", result);
    // task 6..end..

    // task 7 Option From
    let result = meaning_of_life_and_existence();
    println!("The answer is: {:?}", result.unwrap());
    // task 7..end..

    // task 8 From Clones non-copy types
     let a = [vec![1], vec![2], vec![3]];
    let slice = &a[..];
    let _v = Vec::from(slice);

    println!("v: {:?}", a);
    // task 8..end..

}

// task 5 converting integers to larger sizes

pub fn bigger(v: Vec<u8>) -> Vec<u16> {
    let mut v2: Vec<u16> = vec![];
    for i in v {
        let v1 = u16::from(i);
        v2.push(v1);
    }
    v2
    
}
// task 5..end..

// task 6 from bool
pub fn zero_one(b: bool) -> i32 {
    i32::from(b)
}
// task 6..end..

// task 7 Option From

pub fn meaning_of_life_and_existence() -> Option<i32> {
   Option::from(42)
}
// task 7..end..
