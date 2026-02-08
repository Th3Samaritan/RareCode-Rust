use std::collections::HashSet;
fn main() {
    // task 1 TurboFish Introduction
    let numbers = vec![1, 2, 3, 4, 5];
    let result = sum_of_slice(&numbers);
    println!("{}", result);
    // task 1..end..

    // task 2 Collecting one type to another
      let v = vec![1, 2, 3, 4, 5];
    let set = v.into_iter().collect::<HashSet<i32>>();
    println!("{:?}", set);
    // task 2..end..

    // task 3 Exercise: Sum times 2
     let a = [1, 2, 3, 4, 5];
    let result = sum_x2(&a);
    println!("Sum × 2 = {}", result);

    // task 3..end..

    // task 4 Empty Collection
     println!("{:?}", HashSet::<i32>::new());
    println!("{:?}", Vec::<i32>::new());
    // task 4..end..

    // task 5 Exercise 2D Sum
    let a = [[1,2,3],[4,5,6],[7,8,9]];
	
	let result = two_d_sum(&a);
	
	println!("{}", result)
    // task 5..end..

    // task 6 Exercise: Double Product
    let v = vec![1, 2, 3];
    let result = double_product(&v);
    println!("{}", result);
    // task 6..end..
}

// task 1 TurboFish Introduction
fn sum_of_slice(slice: &[i32]) -> bool {
    if slice.iter().sum::<i32>() > 10 {
        return true;
    }
    false
}
// task 1..end..

// task 3 Exercise: Sum times 2

pub fn sum_x2(s: &[i32]) -> i32 {
    s.iter().sum::<i32>() * 2
}
// task 3..end..

// task 5 Exercise: 2D Sum
pub fn two_d_sum(a: &[[i32; 3]; 3]) -> i32 {
    let mut b = 0;
    for i in a {
        b += i.iter().sum::<i32>();
    }
    b
}
// task 5..end..

// task 6 Exercise: Double Product
pub fn double_product(v: &[i32]) -> i32 {
    v.iter().product::<i32>() * 2
}