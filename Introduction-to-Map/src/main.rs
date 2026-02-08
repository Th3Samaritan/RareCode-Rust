fn main() {
    // task 1 Rush Map
    let v = vec![1, 2, 3, 4, 5];
    let result = v.into_iter().map(increment).collect::<Vec<i32>>();
    println!("{:?}", result);
    // task 1..end..

    // task 2 Closure
     let increment = |x| { x + 1 };

    let v = vec![1, 2, 3, 4, 5];
    let result = v.into_iter().map(increment).collect::<Vec<i32>>();
    println!("{:?}", result);
    // task 2..end..

    // task 3 Multi-line closure
     let mul_of_2_or_3 = |x: i32| {
        if x % 2 == 0 {
            return Some(2);
        } else if x % 3 == 0 {
            return Some(3);
        }
        None
    };

    let result = (0..=10)
        .into_iter()
        .map(mul_of_2_or_3)
        .collect::<Vec<Option<i32>>>();
    println!("{:?}", result);
    // task 3..end..

    // task 4 Closure in map
     let v = [1, 2, 3];
    let sum_of_squares: i32 = v.into_iter().map(|x| x*x).sum();
    println!("{}", sum_of_squares);
    // task 4..end..

    // task 5 Exercise: Less Than 10
     let v = vec![1, 2, 10];
    let result = less_than_ten(v);
    println!("{:?}", result);
    // task 5..end..

    // task 6 Exercise: Convert Up All
      let v = vec![1, 2, 3];
    let result = convert_up(v);
    println!("{:?}", result);
    // task 6..end..

    // task 7 Exercise: Unwrap Inc Wrap
     let v = vec![Some(5), Some(15), None];
    let result = inc_option(v);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Exercise: Double Deref
     let v = vec![&&1,&&2,&&3];
    let result = convert_double_ref_to_owned(v);
    println!("{:?}", result);
        // task 8..end..

    // task 9 Example: Inc Square Sum
    let v = vec![1, 2, 3];
    let result = inc_square_sum(v);
    println!("{}", result);
    // task 9..end..

    // task 10 Exercise: One Line 2D Sum
       let grid = [[0, 0, 0], [1, 1, 1], [2, 2, 2]];
    let result = grid_sum(grid);
    println!("{}", result);
    // task 10..end..
}

// task 1 Rush Map
pub fn increment(x: i32) -> i32 {
    x + 1
}
// task 1..end..

// task 5 Exercise: Less Than 10

pub fn less_than_ten(v: Vec<i32>) -> Vec<bool> {
  v.into_iter().map(|x| x < 10 ).collect()
}
// task 5..end..

// task 6 Exercise: Convert Up All

pub fn convert_up(v: Vec<i32>) -> Vec<i128> {
 let result:  Vec<i128> = v.into_iter().map(|x|i128::from(x)).collect();
 result
}

// task 6..end..

// task 7 Exercise: Unwrap Inc Wrap
pub fn inc_option(v: Vec<Option<i32>>) -> Vec<Option<i32>> {
    v.into_iter()
        .map(|x| {
                  if x.is_some() {
                Some(x.unwrap() + 1)
            } else {
                None
            }
        })
        .collect()
}
// task 7..end..

// task 8 Exercise: Double Deref
pub fn convert_double_ref_to_owned(v: Vec<&&i32>) -> Vec<i32> {
    v.into_iter().map(|x| **x).collect()
}
// task 8..end..

// task 9 Example: Inc Square Sum
pub fn inc_square_sum(v: Vec<i32>) -> i32 {
    v.into_iter().map(|x| {x+1}).map(|x| {x*x}).sum()
}
// task 9..end..

// task 10 Exercise: One Line 2D Sum
pub fn grid_sum(grid: [[i32; 3]; 3]) -> i32 {
     grid.into_iter()
        .map(|row| row.into_iter().sum::<i32>())
        .sum()
}
// task 10..end..