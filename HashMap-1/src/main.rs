use std::collections::HashMap;

// task 6
pub fn does_k_exist(hm: HashMap<i32, i64>, k: &i32) -> i32 {
    // your code here
    if hm.get(k).is_some(){
        1
    } else {
        0
    }
}

// task 6..end..

// task 7
pub fn vector_to_index_map(v: Vec<i32>) -> HashMap<i32, usize> {
    let mut hm = HashMap::new();
    for i in 0..v.len(){
        hm.insert(v[i], i);
    }
    hm
}

// task 7..end..

// task 8

pub fn evenness(v: Vec<i32>) -> HashMap<i32, bool> {
    let mut hm = HashMap::new();
    for i in 0..v.len(){
        if v[i] % 2 == 0{
            hm.insert(v[i], true);
        }else{
            hm.insert(v[i], false);
        }
    }hm
}
// task 8..end..

// task 9 HashMap Override
pub fn first_indices(v: Vec<i32>) -> HashMap<i32, usize> {
    let mut hm = HashMap::new();

    for i in 0..v.len(){
       
    if hm.get(&v[i]).is_none(){
         hm.insert(v[i], i);
    } 
 }hm
}

// task 9..end..

// task 10
pub fn count_occurrences(v: Vec<i32>) -> HashMap<i32, i32> {
    // your code here
    let mut hm = HashMap::new();
    for i in 0..v.len(){
        if hm.get(&v[i]).is_none(){
            hm.insert(v[i], 1);
        }else {
            let current_count = hm.get(&v[i]).unwrap();
            hm.insert(v[i], current_count + 1);
        }
    } hm
}

// task 10..end..

// task 1 HashMap

fn main() {
    // task 1 HashMap
    let hm: HashMap<usize, u32> = HashMap::new();
    println!("{:?}", hm); 
    // task 1..end..

    // task 2 HashMap Insert Operations
    let mut hm: HashMap<i32, bool> = HashMap::new();
    hm.insert(2, true);
    hm.insert(3, false);
    hm.insert(4, true);
    hm.insert(5, false);
    println!("{:?}", hm);
    // task 2..end..

    // task 3 Mutable HashMap
      let mut hm: HashMap<i32, bool> = HashMap::new();
    hm.insert(1, false);
    hm.insert(2, false);
    hm.insert(1, true); 

    println!("{:?}", hm);
    // task 3..end..

    // task 4 Reference Type in HashMap
     let mut hm: HashMap<i32, i32> = HashMap::new();

    hm.insert(2, 5);
    hm.insert(4, 6); 
    println!("{:?}", hm.get(&5));
    // task 4..end..

    // task 5 Vector in HashMap
     let data = vec![
        (10, 5),
        (20, 5), 
        (30, 4),
    ];

    let map: HashMap<i32, i64> = data.into_iter().collect();

    println!("{:?}", map);
    // task 5..end..

    // task 6 HashMap with Functions
    let mut hm = HashMap::new();
    hm.insert(1, 1);
    hm.insert(2, 4);
    hm.insert(3, 9);

    let result_exists = does_k_exist(hm.clone(), &3); 
    println!("Key 3 exists: {:?}", result_exists);

    let result_not_exists = does_k_exist(hm.clone(), &5);
    println!("Key 5 exists: {:?}", result_not_exists);
    // task 6..end..

    // task 7 HashMap Vector to key-value
    let data1 = vec![5, 7, 5, 9];
    let result1 = vector_to_index_map(data1);
    println!("Input: vec![5, 7, 5, 9], Output: {:?}", result1); // Expected: {5: 2, 7: 1, 9: 3}

    let data2 = vec![1, 2, 3, 4, 5];
    let result2 = vector_to_index_map(data2);
    println!("Input: vec![1, 2, 3, 4, 5], Output: {:?}", result2); // Expected: {1:0, 2:1, 3:2, 4:3, 5:4}

    let data3 = vec![10, 10, 10];
    let result3 = vector_to_index_map(data3);
    println!("Input: vec![10, 10, 10], Output: {:?}", result3);
    // task 7..end..

    // task 8 HashMap Equation
     let nums1 = vec![1, 2, 3, 4];
    let result1 = evenness(nums1);
    println!("Input: vec![1, 2, 3, 4], Output: {:?}", result1); // Expected: {1: false, 2: true, 3: false, 4: true}

    let nums2 = vec![4, 7, 4, 9];
    let result2 = evenness(nums2);
    println!("Input: vec![4, 7, 4, 9], Output: {:?}", result2); // Expected: {4: true, 7: false, 9: false}

    let nums3 = vec![0, -2, -3];
    let result3 = evenness(nums3);
    println!("Input: vec![0, -2, -3], Output: {:?}", result3);
    // task 8..end..

    // task 9 HashMap Override
     let nums1 = vec![10, 20, 10, 30, 20];
    println!(
        "Input: {:?}, Output: {:?}",
        nums1,
        first_indices(nums1.clone())
    );

    let nums2 = vec![7, 7, 7];
    println!(
        "Input: {:?}, Output: {:?}",
        nums2,
        first_indices(nums2.clone())
    );

    let nums3 = vec![9, 8, 7, 6];
    println!(
        "Input: {:?}, Output: {:?}",
        nums3,
        first_indices(nums3.clone())
    );

    let nums4 = vec![5, 3, 5, 10, 3, 7];
    println!(
        "Input: {:?}, Output: {:?}",
        nums4,
        first_indices(nums4.clone())
    );
    // task 9..end..

    // task 10 Count Occurence HashMap
    let data1 = vec![10, 15, 10, 20, 15, 25, 10];
    let map1 = count_occurrences(data1.clone());
    println!(
        "Input: vec![10, 15, 10, 20, 15, 25, 10], Output: {:?}",
        map1
    );
    // Expected: {10: 3, 15: 2, 20: 1, 25: 1}

    let data2 = vec![1, 1, 1, 1, 1];
    let map2 = count_occurrences(data2.clone());
    println!("Input: vec![1, 1, 1, 1, 1], Output: {:?}", map2);
    // Expected: {1: 5}

    let data3: Vec<i32> = vec![];
    let map3 = count_occurrences(data3.clone());
    println!("Input: vec![], Output: {:?}", map3);
    // task 10..end..
}
