use std::ops::Range;
 use std::collections::HashSet;
// task 3.
pub fn collect_range_to_vector(start: usize, end: usize) -> Vec<usize> {
    // Your code her
    let v1: Vec<usize> = (start..end).collect();
    v1
}
// task 3..end..

// task 4
pub fn collect_range_to_set(start: i32, end: i32) -> HashSet<i32> {
    let v1: HashSet<i32> = (start..end).collect();
    v1
}
// task 4..end..

// task 5
pub fn range_step_to_30(step: i32) -> HashSet<i32> {
    (0..30).step_by(step as usize).collect()
} 
// task 5..end..

// task 6
pub fn collect_and_sum_range(start: i32, end: i32) -> Vec<i32> {
    // your code here
    let mut v = Vec::new();
    let mut total = 0;
    for i in start..=end {
        total += i;
         v.push(i);
    }
    v.push(total);
    v
}
// task 6..end..

// task 7 Range Reverse
pub fn reverse_range(start: i32, end: i32) -> Vec<i32> {
    let v: Vec<i32> = (start..=end).rev().collect();
    v
}

// task 7..end..

// task 8 Range Reverse Step
pub fn reverse_range_with_step(step_by: usize) -> Vec<i32> {
    // Your code here
    let v: Vec<i32> = (0..=10).rev().step_by(step_by).collect();
    v
}
// task 8..end..

// task 9 Range Odd Descending
pub fn odd_descending(end: i32) -> Vec<i32> {
    // You code here
    let v:Vec<i32> = (1..end).step_by(2).rev().collect();
    v
}
// task 9..end..

// task 10
pub fn collect_range_values(r: Range<usize>) -> HashSet<usize> {
    // Your code here
    r.collect()
}
// task 10..end..

// task 11 Sum Even Range
pub fn sum_even_range(my_range: Range<i32>) -> i32 {
    // Your code here
     let mut even = 0;
    for i in my_range{
        if i % 2 == 0 {
            even += i;
        } 
    } even
}
// task 11..end..

fn main() {
    // task 1 Introduction to Ranges
    let my_range = 0..10;
    println!("{:?}", my_range); // print my_range

    // task 1..end..

    // task 2 Rust Range and Loop
     println!("Looping with implicit into_iter():");
    for i in 0..10 {
        println!("{}", i);
    }

    println!("Looping with explicit into_iter():");
    // TODO convert the range to an iterator with (0..10).into_iter()
    for i in (0..10).into_iter() {
        println!("{}", i);
    }
    // task 2..end..

    // task 3 Range Silent Conversion to Iterators
    let v: Vec<usize> = (0..10).collect(); // Example from description
    println!("{:?}", v);

    let result = collect_range_to_vector(3, 7);
    println!("{:?}", result);
    // task 3..end..

    // task 4 Range into Set
     let v: HashSet<u32> = (0..10).collect(); // Example from description
    println!("{:?}", v);

    let result = collect_range_to_set(0, 10);
    println!("{:?}", result); // Expected: {0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
    // task 4..end..

    // task 5 Ranges Step
     let v: Vec<i32> = (0..10).step_by(2).collect(); // Example from description
    println!("{:?}", v); // [0, 2, 4, 6, 8]

    let result = range_step_to_30(5);
    println!("{:?}", result); // Expected: {0, 5, 10, 15, 20, 25}
    // task 5..end..

    // task 6 Range inclusive end value
        // Example from description for general inclusive range
    for i in 1..=5 {
        println!("{}", i);
    }

    let result = collect_and_sum_range(4, 8);
    println!("{:?}", result); // Expected: [4, 5, 6, 7, 8, 30
    // task 6..end..

    // task 7 Range Reverse
     // Example from description
    let v1: Vec<i32> = (8..=4).rev().collect(); // ❌ Wrong
    let v2: Vec<i32> = (4..=8).rev().collect(); // ✅ Correct
    println!("V1: {:?}, V2: {:?} ", v1, v2); // V1: [], V2: [8, 7, 6, 5, 4]

    let result = reverse_range(2, 7);
    println!("{:?}", result); // Expected by example: [7, 6, 5, 4, 3, 2]
    // task 7..end..

    // task 8 Range Reverse Step
    // Example from description
    let a: Vec<i32> = (1..9).step_by(3).rev().collect();
    println!("{:?}", a); // [7, 4, 1]

    let b: Vec<i32> = (1..9).rev().step_by(3).collect();
    println!("{:?}", b); // [8, 5, 2]

    println!("{:?}", reverse_range_with_step(2)); // Expected: [10, 8, 6, 4, 2, 0]
    // task 8..end..

    // task 9 Range Odd Descending
    let result = odd_descending(20);
    println!("{:?}", result); // Expected: [19, 17, 15, 13, 11, 9, 7, 5, 3, 1]
    // task 9..end..

    // task 10 Range as a Type
    // Example from description
    let my_range: Range<i32> = 0..10;
    println!("{:?}", my_range);

    let values = collect_range_values(5..10);
    println!("{:?}", values); // Expected: {5, 6, 7, 8, 9}
    // task 10..end..

    // task 11 Sum Even Range
    let total = sum_even_range(0..5); // 0, 1, 2, 3, 4. Even: 0, 2, 4. Sum = 6
    println!("Sum: {}", total); // Expected: 6
    // task 11..end..
}
