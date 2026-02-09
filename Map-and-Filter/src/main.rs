use std::collections::{HashMap, HashSet};
fn main() {
    // task 1 map and filter on .iter()
    let a: Vec<usize> = vec![0, 1, 3];

    let _result1 = a.iter().map(|x| accept(*x)).collect::<Vec<bool>>();
    let _result2 = a.iter().filter(|x| accept(**x)).collect::<Vec<&usize>>();
    // task 1..end..

    // task 2 map on .values()
     let map = HashMap::from([(0, 1), (2, 3)]);
    let result = sum_values_inc(map);
    println!("{}", result);
    // task 2..end..

    // task 3 .iter() on a reference
     let mut set = HashSet::from([1, 2, 3]);
    let result = sum_odd(&mut set);
    println!("{}", result);
    // task 3..end..

    // task 4 map on a collection of references
      let mut set = HashSet::from([&1, &2, &3]);
    let result = sum_odd(&mut set);
    println!("{}", result);
    // task 4..end..

    // task 5 map on a slice
     let a = [1,2,3,4,5];

    let result = sum_of_squares_tail(&a);
    println!("{}", result)
    // task 5..end..

    // task 6 Filter on an iterator created from a slice
     let a = vec![1, 3, 3, 4, 5, 6, 7, 8, 9];
    let result = odds(&a);
    println!("{}", result);
    // task 6..end..
}

// task 2 map on .values()
pub fn sum_values_inc(map: HashMap<i32, i32>) -> i32 {
    map.values().map(|&x| x+1).sum::<i32>()
    }
// task 2..end..

// task 3

pub fn sum_odd(set: &mut HashSet<i32>) -> i32 {
    set.iter().filter(|x| logic(**x)).sum()
}

pub fn logic(x: i32) -> bool {
    x % 2 != 0
}

// task 3..end..

// task 4 map on a collection of references

pub fn sum_odd(set: &mut HashSet<&i32>) -> i32 {
    // you will need to make an additional change so `.sum()` works
    set.iter().filter(|&&&x| logic(x)).map(|&&x| x).sum()
}
// task 4..end..

// task 5 map on a slice

pub fn sum_of_squares_tail(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    arr[1..].iter().map(|&x| x*x).sum()
}
// task 5..end..

// task 6 Filter on an iterator created from a slice

pub fn odds(a: &[i32]) -> usize {
    a.iter().filter(|&&x| x % 2 != 0).count()
}
// task 6..end..