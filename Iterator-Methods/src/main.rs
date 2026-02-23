use std::collections::HashSet;
fn main() {
    // task 1 .sum()
    let v = vec![1, 2, 3];

    let result: i32 = v.into_iter().sum();
    println!("{}", result);
    // task 1..end..

    // task 2
     let v = vec![1, 2, 3];

    let result: i32 = v.into_iter().sum();
    println!("{}", foo(result));
    // task 2..end..

    // task 3
      let s = HashSet::from([1, 2, 3]);

    let result = sum_of_set(s);
    println!("{}", result);
    // task 3..end..

    // task 4
     let s = HashSet::from([1, 2, 3]);

    let result = min_of_set(s);
    println!("{}", result);
    // task 4..end..

    // task 5
     let s = HashSet::from([1, 2, 3]);

    let result = max_of_set(s);
    println!("{}", result);
    // task 5..end..

    // task 6
     let s = HashSet::from([1, 2, 3]);
    let result = product_of_set(s);
    println!("{}", result);
    // task..end..

    // task 7
      let vv = vec![vec![1, 2, 3], vec![4, 5, 6], vec![9, 8, 7]];

    let result = max_of_each(vv);
    println!("{:?}", result);
    // task 7..end..

    // task 8 .nth()
    let v = vec![1, 2, 3];
    let mut my_iter = v.clone().into_iter();

    println!("{}", v[0] == my_iter.nth(0).unwrap() /* add .nth(0).unwrap() */); 
    // task 8..end..

    // task 9 .count()
     let v = vec![1, 2, 3];
    let my_iter = v.clone().into_iter();

    println!("{}", v.len() == my_iter.count() /* add .count() */); 
    // task 9..end..

    // task 10
      let v = vec![1, 2, 3];
    let result = get_last(v);
    println!("{}", result);
    // task 10..end..
}

// task 2 Type Infer
pub fn foo(x: i32) -> i32 {
    x
}
// task 2..end..

// task 3 HashSet Sum
pub fn sum_of_set(s: HashSet<i32>) -> i32 {
    // your code here
    let s1 = s.into_iter();
    s1.sum()
}
// task 3..end..

// task 4 .min()
pub fn min_of_set(s: HashSet<i32>) -> i32 {
    // your code here
    s.into_iter().min().unwrap() // add .min().unwrap() to the end
}
// task 4..end..

// task 5 .max()
pub fn max_of_set(s: HashSet<i32>) -> i32 {
    // your code here
    s.into_iter().max().unwrap() // add .max().unwrap() to the end
}
// task 5..end..

// task 6 .product()
pub fn product_of_set(s: HashSet<i32>) -> i32 {
    // Your code here
    s.into_iter().product() // add .product() to the end
}
// task 6..end..

// task 7 Max Vector
pub fn max_of_each(vv: Vec<Vec<i32>>) -> Vec<Option<i32>> {
    // your code here
    let mut v1 = Vec::new();
    let vi = vv.into_iter();
    for i in vi{
            let v2 = i.into_iter().max();
            v1.push(v2);
    }v1
}
// task 7..end..

// task 10 Exercise: Get Last Element with Function
pub fn get_last(v: Vec<i32>) -> i32 {
    // your code here
    v.into_iter().last().unwrap()
}
// task 10..end..