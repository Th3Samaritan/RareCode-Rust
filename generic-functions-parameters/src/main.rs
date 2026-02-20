// task 1 Generic Types Introduction
pub fn is_even_length<T>(v: Vec<T>) -> bool {
    v.len() % 2 == 0
}
// task 1..end..

// task 2 Exercise: Reversing an vector of arbitrary type
pub fn rev<T>(v: Vec<T>) -> Vec<T>{
    v.into_iter().rev().collect()
}
// task 2..end..

// task 3 Exercise: First Two
pub fn first_two<T>(v: Vec<T>)->Option<Vec<T>>{
    if v.len() < 2 {
       return None;
    }
   Some(v.into_iter().take(2).collect())
}
// task 3..end..

// task 4
pub fn first_to_last<T>(mut v: Vec<T>)-> Vec<T>{
    if v.is_empty() {
        return v;
    }
    let first = v.remove(0);
    v.push(first);
    v
}
// task 4..end..

// task 5 Exercise: Take Last N
pub fn take_last_n<T>(v: Vec<T>, n: usize) -> Vec<T> {
    v.into_iter().rev().take(n).rev().collect()
}
// task 5..end..

// task 6 Exercise: Tuple Reverse
pub fn rtup<T>(t: (T, T)) -> (T, T){
    (t.1, t.0)
}
// task 6..end..

// task 7 Accepting more than one arbitrary type
pub fn rtup<T, U>(t: (T, U)) -> (U, T) {
    (t.1, t.0)
}
// task 7..end..

// task 8
pub fn len_compare<T, U>(first: Vec<T>, second: Vec<U>) -> bool{
    first.len() > second.len()
}
// task 8..end..

// task 9
pub fn pair_lengths<T, U>(first: Vec<T>, second: Vec<U>) -> (usize, usize) {
    (first.len(), second.len())
}
// task 9..end..

// task 10 Exercise: Both Empty
pub fn both_empty<T, U> (first: Vec<T>, second: Vec<U>)->bool {
    first.is_empty() && second.is_empty()
}
// task 10..end..

fn main() {
    // task 1 Generic Types Introduction
    let v = vec![1,2,3];
    let result = is_even_length(v);
    println!("{:?}", result);
    // task 1..end..

    // task 2 Exercise: Reversing an vector of arbitrary type
    let v = vec![1,2,3,4];
    let result = rev(v);
    println!("{:?}", result);
    
    let v = vec!['a', 'b', 'c', 'd'];
    let result = rev(v);
    println!("{:?}", result);
    // task 2..end..

    // task 3 Exercise: First Two
     let v = vec![1,2,3,4];
    let result = first_two(v);
    println!("{:?}", result);
    
    let v = vec![1.0,2.0,3.0,4.0];
    let result = first_two(v);
    println!("{:?}", result);
    // task 3..end..

    // task 4 Exercise: Move first to last
      let v = vec![1,2,3,4];
    let result = first_to_last(v);
    println!("{:?}", result);

    let v = vec!["a".to_string(), "b".to_string()];
    let result = first_to_last(v);
    println!("{:?}", result);

    let v = vec![5.3];
    let result = first_to_last(v);
    println!("{:?}", result);

    let v: Vec<i32> = vec![];
    let result = first_to_last(v);
    println!("{:?}", result);
// task 4..end..

// task 5 Exercise: Take Last N
 let v = vec![1,2,3,4];
    let result = take_last_n(v, 2);
    println!("{:?}", result);
    // task 5..end..

// task 6 Exercise: Tuple Reverse
 let t = (1, 4);
    let result = rtup(t);
    println!("{:?}", result);
    
    let t = (1.0, 4.0);
    let result = rtup(t);
    println!("{:?}", result);
    // task 6..end..

    // task 7 Accepting more than one arbitrary type
     let t = (1, true);
    let result = rtup(t);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Exercise: Length Compare
    let result = len_compare([1,2,3].to_vec(), [true, false].to_vec());
    println!("{:?}", result);
    // task 8..end..

    // task 9 Exercise: pair_lengths
    let a = vec![1, 2, 3];
    let b = vec!['x', 'y'];
    println!("{:?}", pair_lengths(a, b));
    // task 9..end..

    // task 10 Exercise: Both Empty
     let a: Vec<String> = Vec::new();
    let b: Vec<char> = Vec::new();
    let result = both_empty(a, b);
    println!("{:?}", result);
    // task 10..end..
}