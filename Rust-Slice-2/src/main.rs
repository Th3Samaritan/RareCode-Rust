use std::collections::HashSet;
fn main() {
    // task 1 Helpful Syntax
    let a = [1,2,3,3,4,5];

    let result = up_to_including_k(&a, 3);
    println!("{:?}", result); // Output: [1, 2, 3]
    // task 1..end..

    // task 2 Exercise: Last N
    let a = [1,2,3,3,4,5];

    let result = k_to_end(&a, 3);
    println!("{:?}", result);
    // task 2..end..

    // task 3 Slice of vector or array containing references
     let v = vec![&1, &2, &3, &4, &5];
    let result = slice_of_ref_to_vec(&v);
    println!("{:?}", result);
        // task 3..end..

    // task 4 Exercise: type of e
     let a = [&1, &2, &3];
    let sl = &a[..]; // slice of the entire vector

    for e in sl.iter() {
        accept(**e);
    }
    // task 4..end..

    // task 5 First 2 into set
     let a = [&1,&2,&3,&4,&5];
    let result = first_two_as_set(&a);
    println!("{:?}", result);
    // task 5..end..

    // task 7 Exercise: Slice to reversed vector
    let v = vec![1,2,3];
    let result = reversed(&v[..]);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Exercise: Forward and Backward
     let v = vec![1,2,3];
    let result = forwards_and_backwards(&v[..]);
    println!("{:?}", result);
    // task 8..end..

    // task 9 Exercise: Reverse the middle
     let mut v = vec![1, 2, 3, 4, 5, 6];
    reverse_the_middle(&mut v);
    println!("{:?}", v);
    // task 9..end..
}

// task 1 Helpful Syntax
pub fn up_to_including_k(a: &[i32], k: i32) -> Vec<i32> {
    for (i, &val) in a.iter().enumerate(){
        if val == k {
            return a[..=i].iter().copied().collect();
        }
    }
    vec![]
}
// task 1..end..

// task 2 Exercise: Last N

pub fn k_to_end(a: &[i32], k: i32) -> Vec<i32> {
    for (i, &val) in a.iter().enumerate().rev(){
         if val == k {
            return a[i..].iter().copied().collect();
        }
    }
    vec![]
    
}
// task 2..end..

// task 3 Slice of vector or array containing references

pub fn slice_of_ref_to_vec(a: &[&i32]) -> Vec<i32> {
		a.into_iter().copied().copied().collect()
}
// task 3..end..

// task 5 First 2 into set

pub fn first_two_as_set(slr: &[&i32]) -> Option<HashSet<i32>> {
    if slr.len() < 2 {
        return None;
    }
    let set = slr[..2].iter().copied().copied().collect();
    Some(set)
}
// task 5..end..

// task 7 Exercise: Slice to reversed vector

pub fn reversed(v: &[i32]) -> Vec<i32> {
    v.iter().rev().copied().collect()
}
// task 7..end..

// task 8 Exercise: Forward and Backward

pub fn forwards_and_backwards(v: &[i32]) -> Vec<i32> {
    let mut forward: Vec<i32> = v.iter().copied().collect();
    let reversed: Vec<i32> = v.iter().rev().copied().collect();
    forward.extend(reversed);
    forward
}
// task 8..end..

// task 9 Exercise: Reverse the middle

pub fn reverse_the_middle(v: &mut Vec<i32>) {
    if v.len() <= 3 {
        return;
    }
    let vlen = v.len();
    let middle = &mut v[1..vlen - 1];

    let midlen = middle.len();
    for i in 0..midlen / 2 {
        let tmp = middle[i];
        middle[i] = middle[midlen - 1 - i];
        middle[midlen - 1 - i] = tmp;
    }
}

// task 9..end..