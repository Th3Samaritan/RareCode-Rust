use std::collections::HashSet;
// task 7 Exercise of vectors inside a tuple reference
fn get_num(pair: &(Vec<i32>, i32)) -> (Vec<i32>, i32) {
	pair.clone() // remove the * and .clone() this
}
// task 7..end..

// task 9
pub fn flatten_vector(group: &Vec<Vec<i32>>) -> HashSet<i32> {
	// your code here
    let mut v1 = HashSet::new();
    for i in group{
        for j in i{
            v1.insert(*j);
        }
    }v1
}

// task 9..end..
fn main() {
    // task 1 Vector Dereference Compilation Failure
    let x = vec![1, 2, 3];
    let ref_x = &x;

    foo(ref_x.clone()); // ❌ this won't compile. Replace with ref_x.clone()
    // task 1..end..

    // task 2
     let a = 1;
    let b = 2;
    let c = 3;

    let numbers: Vec<&i32> = vec![&a, &b, &c];

   let values = collect_values(numbers);
    println!("{:?}", values);
    // task 2..end..

    // task 3
     let a = 10;
    let b = 20;
    let c = 30;

    let refs = vec![&a, &b, &c];

    let result = return_owned_vector(&refs);
    println!("{:?}",result);
    // task 3..end..

    // task 4
     let a = vec![50, 10, 25];
    let b = vec![100, 400];
    let c = vec![150, 600, 700];

    let group_vec: Vec<&Vec<i32>> = vec![&a, &b, &c];
    
    let append = append_sum(group_vec);
    println!("{:?}", append); 
    // task 4..end..

    // task 5
     let p = (2, 5);
    println!("{:?}", swap_and_double(&p));
    // task 5..end..

    // task 6
     let a = 100;
    let b = false;
    let p = (&a, &b);
    println!("{:?}", take_copy(p));
    // task 6..end..

    // task 7 Exercise of vectors inside a tuple reference
    let p = (vec![4, 5], 9);
    println!("{:?}", get_num(&p));
    // task 7..end..

    // task 8
     let t = (vec![1, 2, 3], vec![4, 5]);
    println!("{:?}", sum_tuple(&t));
    // task 8..end..

    // task 9 Exercise: Flatten a reference to a nested vector
     let group = vec![vec![1], vec![2], vec![3], vec![4]];

    let flat = flatten_vector(&group);
    println!("{:?}", flat);
    // task 9..end..
}

// task 1 Vector Dereference Compilation Failure
fn foo(v: Vec<i32>) -> usize {
    v.len()
}
// task 1..end..

// task 2 
pub fn collect_values(input: Vec<&i32>) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();

    for value in input.into_iter() {
        // dereference each value and push it to values
        // your code here
    }

    values
}
// task 2..end..

// task 3 Practice cloning and dereferencing
pub fn return_owned_vector(input: &Vec<&i32>) -> Vec<i32> {
    let v = dereference_values_from_vector(input); // Fix this
    v
}

pub fn dereference_values_from_vector(v: Vec<&i32>) -> Vec<i32> {
    let mut values = Vec::new();

    for val in v {
        values.push(val); // Fix this
    }
    values
}

// task 3..end..

// task 4 Cloning elements in nested collections
pub fn append_sum(v: Vec<&Vec<i32>>)->Vec<i32>{
    let mut new_v = v[2].clone();
    
    let mut result = 0;

    for i in v[2]{
        result +=i;
    }

    new_v.push(result);
    new_v
}

// task 4..end..

// task 5 Dereferencing a tuple
pub fn swap_and_double(pair: &(i32, i32)) -> (i32, i32) {
    let (a, b) = *pair;
    (b * 2, a * 2) // Swap and double
}

// task 5..end..

// task 6 References inside a tuple
pub fn take_copy(pair: (&i32, &bool)) -> (i32, bool) {
    (*pair.0, *pair.1) // dereference the &i32 and the &bool
}
// task 6..end..

// task 8 Exercise of references with vectors and tuples
pub fn sum_tuple(input: &(Vec<i32>, Vec<i32>)) -> (i32, i32) {
	// your code here
    let mut v1 = 0;
    for i in &input.0{
        v1 += *i;
    }
    let mut v2 = 0;
    for i in &input.1{
        v2 += *i;
    }
    (v1, v2)
}
// task 8..end..