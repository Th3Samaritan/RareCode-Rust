use std::collections::{HashSet, HashMap};
fn main() {
    // task 1 introduction to .filter()
    let v = vec![1, 2, 3, 4];
    
    let even_v = v.into_iter().filter(|x| *x % 2 == 0).collect::<Vec<i32>>();
    println!("{:?}", even_v); // [2, 4]
    // task 1..end..

    // task 2 destructuring with .filter
     let v = vec![5,6,7,8,9,10,11];
    
    let result = keep_less_than_ten(v);
    println!("{:?}", result);
    // task 2..end..

    // task 3 Exercise: Remove Nones
    let v = vec![Some(10), None, Some(42), None, None];
    
    let result = remove_nones(v);
    println!("{:?}", result);
    // task 3..end..

    // task 4 When not to destructure
     let v = vec![1, 2, 3, 4];
    let set = HashSet::from([2, 3]); 
    
    let result = keep_elements_in_set(v, set);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Exercise: filter combined with map
     let v = vec![Some(10), None, None, Some(20)];
    
    let result = unwrap_all_somes(v);
    println!("{:?}", result);
    // task 5..end..

    // task 6 Exercise: Remove vectors that contain 0
     let v = vec![vec![1,2,0], vec![1,1,1], vec![3, 0, 3], vec![2]];
    let result = remove_zero_rows(v);
    println!("{:?}", result);
        // task 6..end..

    // task 7 Exercise: keep vectors who sum less than or equal k
      let v = vec![vec![101], vec![50, 50], vec![80, 20, 10], vec![60, 40], vec![]];
    let k = 100;
    let result = filter_vec_sum_lte_k(v, k);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Exercise: discard numbers that cannot be downcast
     let a: [u16; 5] = [235, 254, 255, 256, 257];
    let result: Vec<u8> = downcasted(a);
    println!("{:?}", result);
    // task 8..end..

    // task 9 Exercise: Vectors shorter than k
    let v = vec![vec![1,2,3,4], vec![], vec![1,2,3]];
    let k = 4;
    let result = vecs_shorter_than_k(v, k);
    println!("{:?}", result);
    // task 9..end..

    // task 10 Exercise: keep where values in hashmap sum to greater than 10
    let v = vec![1, 2, 3, 4];
    let m = HashMap::from([(1, vec![8,3]), (2, vec![10]), (3, vec![11])]);
    let result = keep_gt_ten(v, &m);
    println!("{:?}", result);
    // task 10..end..
}

// task 2 destructuring with .filter
pub fn keep_less_than_ten(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().filter(|&x| x < 10).collect::<Vec<i32>>()
}
// task 2..end..

// task 3 Exercise: Remove Nones
pub fn remove_nones(v: Vec<Option<i32>>) -> Vec<Option<i32>> {
    v.into_iter().filter(|&x| x.is_some()).collect()
}
// task 3..end..

// task 4 When not to destructure
pub fn keep_elements_in_set(v: Vec<i32>, s: HashSet<i32>) -> Vec<i32> {
    v.into_iter().filter(|x| {
        s.contains(x)
    }).collect()
}
// task 4..end..

// task 5 Exercise: filter combined with map
pub fn unwrap_all_somes(v: Vec<Option<i32>>) -> Vec<i32> {
    v.into_iter().filter(|&x| x.is_some()).map(|x| x.unwrap()).collect()
}
// task 5..end..

// task 6 Exercise: Remove vectors that contain 0
pub fn remove_zero_rows(v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    v.into_iter().filter(|x| {
        !x.contains(&0)
    }).collect::<Vec<Vec<i32>>>()
}
// task 6..end..

// task 7 Exercise: keep vectors who sum less than or equal k
pub fn filter_vec_sum_lte_k(v: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    v.into_iter().filter(|x| {
        x.iter().sum::<i32>() <=k
    }).collect::<Vec<Vec<i32>>>()
}
// task 7..end..

// task 8 Exercise: discard numbers that cannot be downcast
pub fn downcasted(v: [u16; 5]) -> Vec<u8> {
    v.into_iter().filter(|&x| x<=u8::MAX as u16).map(|x| x as u8).collect::<Vec<u8>>()
}
// task 8..end..

// task 9 Exercise: Vectors shorter than k
pub fn vecs_shorter_than_k(v: Vec<Vec<i32>>, k: u8) -> Vec<Vec<i32>> {
    v.into_iter().filter(|x| x.len() < usize::from(k)).collect()
}
// task 9..end..

// task 10 Exercise: keep where values in hashmap sum to greater than 10
pub fn keep_gt_ten(v: Vec<i32>, m: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    v.into_iter().filter(|x| {
        if m.contains_key(x){
            let sum: i32 = m.get(x).unwrap().iter().sum();
            sum > 10
        } else {
            false
        }
    }).collect()
}
// task 10..end..