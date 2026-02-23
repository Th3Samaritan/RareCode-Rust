use std::collections::{HashSet, HashMap};
fn main() {
    // task 1 Introduction to Dereferencing
    let vector = vec![1, 2, 3];
    let result = sum_doubles(&vector);
    println!("{}", result);
    // task 1..end..

    // task 2 Automatic Dereferencing 1
    let x = 10;
    let y = 5;
    let x_ref = &x;
    let y_ref = &y;
    let sum = *x_ref + *y_ref; // Rust auto-dereferences
    println!("Sum: {}", sum);
    // task 2..end..

    // task 3 Automatic Dereferencing 2
        let v = vec![1, 2, 3];
    let result = sum(&v);
    println!("{}", result);
    // task 3..end..

    // task 4 Index Dereference
     let numbers = vec![10, 20, 30];
    let index_ref = &1;
    println!("{}", numbers[*index_ref]); 
    // task 4..end..

    // task 5
    let x = 1;
    println!("{}", increment(x)); // 2
    println!("{}", increment(&x)); // 2
    // task 5..end..

    // task 6 Dereference Compiler Error
      let start = &0;
    let end = &3;

    for i in *start..*end {
        println!("{}", i);
    }
    // task 6..end..

    // task 7 Vector get reference
     // <Do Not Edit>
    let v = vec![1, 2, 3];
    let index = &0;
    // </Do Not Edit>

    // fix the bug
    let result = v.get(*index);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Set .contains() only accepts reference
    // <Do Not Edit>
    let s = HashSet::from([1, 2, 3]);
    let e = 2;
    // </Do Not Edit>

    let result = s.contains(&e);

    println!("{}", result);
    // task 8..end..

    // task 9 HashMap insert no reference
     let mut hm = HashMap::new();
    // <Do Not Edit>
    let k1 = 1;
    let k2 = 2;
    let v1 = 10;
    let v2 = &20;
    // </Do Not Edit>
    hm.insert(&k1, &v1);
    hm.insert(&k2, v2);
    println!("{:?}", hm);
    // task 9..end..

    // task 10 References and Values
     // <Do Not Edit>
    let v = vec![1, 2, 3];
    let except = &1;
    // </Do Not Edit>

    let result = sum_except(&v, *except);

    println!("{}", result);
    // task 10..end..
}

// task 1 Introduction to Dereferencing
pub fn sum_doubles(v: &Vec<i32>) -> i32 {
    let mut s = 0;
    for e in v {
        s += double(*e); // dereference e
    }
    s
}

pub fn double(e: i32) -> i32 {
    e * 2
}
// task 1..end..

// task 3 Automatic Dereferencing 2
pub fn sum(v: &Vec<i32>) -> i32 {
    let mut s = 0;
    for e in v {
        s += *e; // Rust auto-dereferences here. Change to *e. Both work.
    }
    s
}
// task 3..end..

// task 5 When Rust Dereference
pub fn increment<T>(x: T) -> i32
where
    T: std::borrow::Borrow<i32>,
{
    *x.borrow() + 1
}
// task 5..end..