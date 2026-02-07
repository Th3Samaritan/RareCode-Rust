// task 1 Intro to Slice
fn main() {
    let a = [1, 2, 3, 4];

    let my_slice = &a[0..2];
    println!("{:?}", my_slice);
    // task 1..end..

    // task 3 A slice type accepts references to vectors and references to arrays
    let v = vec![1, 2, 3];
    let arr = [1, 2, 3];

    accept(&v);
    accept(&arr);
    // task 3..end..

    // task 4 iter “adds” a layer of reference
     for e in sl.iter() {
		accept(*e);
    }
    // task 4..end..

    // task 5 into_iter() absorbs one outer reference
    for e in sl {
		accept(*e);
    }
    // task 5..end..

    // task 6 Slice to Vector
    let slice = &v[2..4];
    let new_v = create_vec(slice);

    println!("{:?}", new_v);
    // task 6..end..

    // task 7 Increment each
    increment_slice(&mut v[0..2]);

    println!("{:?}", v);
    // task 7..end..

    // task 8 Exercise: Get the maximum value between i and j
    let v: Vec<i32> = vec![4,1,3,2];

    let result = max_between(v.clone(), 1, 3);
    println!("Expect: Some(3) Got: {:?}", result);
    
    let result = max_between(v.clone(), 0, 3);
    println!("Expect: Some(4) Got: {:?}", result);
    
    let result = max_between(v.clone(), 1, 1);
    println!("Expect: None Got: {:?}", result);

    let result = max_between(v.clone(), 1, 4);
    println!("Expect: Some(3) Got: {:?}", result);
    
    let result = max_between(v.clone(), 3, 1);
    println!("Expect: None Got: {:?}", result);

    // task 8..end..

    // task 9 Exercise: Swap I and J
    let mut v = vec![1, 2, 3, 4, 5];

    swap(&mut v[0..5], 0, 4);
    println!("{:?}", v);
    // task 9..end..

    // task 10 Exercise : Last N
      let v = vec![1,2,3,4];
    let result = last_n(&v, 2);
    println!("{:?}", result)
    // task 10..end..

    // task 11 Exercise: First half or Second half
    let v = vec![1, 2, 1];
    let result = first_or_second_half(&v);
    println!("Result: {}", result);
    // task 11..end..

    // task 12 Exercise: Up To Max
       let a = [1, 2, 3, 2, 1];
    let result = up_to_max(&a);
    println!("{:?}", result);
    // task 12..end..

    // task 13 Exercise: Sliding Window Sum Finder
     let a = [1, 2, 3, 4, 1, 2];
    let k = 3;
    let target = 8;

    let result = find_region(&a, k, target);
    println!("{:?}", result);
    // task 13..end..
}

// task 2 Sum of Slice

pub fn slice_sum(s: &[i32])->i32 {
    s.iter().sum()
}
// task 2..end..

// task 3 A slice type accepts references to vectors and references to arrays
pub fn accept(_s: &[i32]) {}
// task 3..end..

// task 4 iter “adds” a layer of reference

fn accept(_z: i32) {}
// task 4..end..

// task 5 into_iter() absorbs one outer reference
fn accept(_z: i32) {}
// task 5..end..

// task 6 Slice to Vector
pub fn create_vec(s: &[i32]) -> Vec<i32> {
    s.iter().copied().collect()
}
// task 6..end..

// task 7 Increment each

pub fn increment_slice(s: &mut [i32]) {
    for v in s{
        *v = *v + 1;
}
}
// task 7..end..

// task 8 Exercise: Get the maximum value between i and j
pub fn max_between(v: Vec<i32>, i: usize, j: usize) -> Option<i32> {
    if i >= j || i > v.len() || j > v.len() {
        return None;
    }
    let slice = &v[i..j];
    slice.iter().max().copied()
}
// task 8..end..

// task 9 Exercise: Swap I and J

pub fn swap(v: &mut[i32], i: usize, j: usize) {
    if i>=j || i > v.len() || j > v.len(){
        return;
    }
        let swp = v[i];
        v[i]=v[j];
       v[j]=swp;
}

// task 9..end..

// task 10 Exercise : Last N

pub fn last_n(sl: &[i32], n: usize) -> Vec<i32> {

    let len = sl.len();
    let start = if n > len { 0 } else {len-n};
    let sub_slice = &sl[start..len];
    sub_slice.into_iter().copied().collect()
}
// task 10..end..

// task 11 Exercise: First half or Second half

pub fn first_or_second_half(sl: &[i32]) -> u8 {
    if sl.len() <= 1 {
    return 0;
}
    let mid = sl.len() /2;
    let first = &sl[0..mid];
    let second = &sl[mid..sl.len()];
    let max_val1 = first.iter().max().unwrap();
    let max_val2 = second.iter().max().unwrap();
    if *max_val1 >= *max_val2 {
         0
    }else {
        1
    }
}
// task 11..end..

// task 12 Exercise: Up To Max

pub fn up_to_max(a: &[i32]) -> Vec<i32> {
    if a.is_empty() {
        return vec![];
    }
     let max_val = *a.iter().max().unwrap();
     let mut end_index = 0;
    for (i, e) in a.iter().enumerate() {
        if *e == max_val {
            end_index = i;
            break; 
        }
    }
    a[0..=end_index].iter().copied().collect()

}
// task 12..end..

// task 13 Exercise: Sliding Window Sum Finder

pub fn find_region(sl: &[i32], k: usize, target: i32) -> Option<usize> {
     if k == 0 || k > sl.len() {
        return None;
    }
    for i in 0..=(sl.len() - k) {
        let window = &sl[i..i + k];
        let sub_sum: i32 = window.iter().sum();

        if sub_sum == target {
            return Some(i);
        }
    }

    None

}
// task 13..end..