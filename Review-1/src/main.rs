use std::collections::HashSet;
// task 1 into_iter()
pub fn check_even(v: Vec<u32>) -> Vec<(u32, bool)> {
    let mut x = Vec::new();
    for i in v.into_iter(){
        let is_even = i%2==0;
        x.push((i, is_even));
    }
    x
}
// task 1..end..

// task 2
pub fn from_set_to_vector(set: HashSet<i32>) -> Vec<i32> {
     set.into_iter().collect()
}
// task 2..end..

// task 3
pub fn from_vector_to_set(values: Vec<i32>) -> HashSet<i32> {
    values.into_iter().collect()
}
// task 3..end..

// task 4
pub fn clone_positive_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    let mut positive_numbers = Vec::new();

        for i in numbers.clone().into_iter(){
            if i >0 {
                positive_numbers.push(i)
            }
        }

    positive_numbers
}
// task 4..end..

// task 5
pub fn increment_second(t: &(i32, i32)) -> (i32, i32) {
    let mut y = t.clone();
    y.1 = y.1 + 1;
    y
}
// task 5..end..

// task 6
pub fn count_even_numbers(numbers: &Vec<i32>)->(usize, usize){
    let mut even_count = 0;
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            even_count += 1;
        }
    }
    (even_count, numbers.len())
}
// task 6..end..

// task 7
pub fn sum_and_cast(numbers: Vec<u32>) -> usize {
    let mut y: u32 = 0;
   for i in numbers.into_iter(){
        y += i;
   }
   y as usize
}
// task 7..end..

// task 8 Putting datatypes into options and putting options into collections
pub fn from_index(v: Vec<i32>, start: usize) -> Option<Vec<i32>> {

    if start>= v.len() {
        return None;
    }

let mut result = Vec::new();

		for i in start..v.len() {
            result.push(v[i]);
        }

    Some(result)
}
// task 8..end..

fn main() {
    // task 1 into_iter()
    let numbers = vec![0, 1, 2, 3]; 
    let result = check_even(numbers);
    println!("{:?}", result); 
    // task 1..end..

    // task 2 into_iter().collect()
     let mut set = HashSet::new();
    set.insert(10);
    set.insert(4);
    set.insert(7);
    let mut result = from_set_to_vector(set);
    result.sort(); 
    println!("{:?}", result); 
    // task 2..end..

    // task 3 Convert Vector to HashSet
    let values = vec![3, 7, 3, 2, 7, 9];
    let result = from_vector_to_set(values);

    println!("{:?}", result);
    // task 3..end..

    // task 4 Cloning a reference
    let input_numbers = vec![3, -4, 10, 0, -7, 8];
    let positives = clone_positive_numbers(&input_numbers);
    println!("Positive numbers: {:?}", positives); 
    // task 4..end..

    // task 5 Making a tuple mutable to change the inside of it
     let my_tuple = &(10, 20);
    let updated_tuple = increment_second(my_tuple);
    println!("Original: {:?}, Updated: {:?}", my_tuple, updated_tuple); 

    let another_tuple = &(0, -5);
    let updated_another_tuple = increment_second(another_tuple);
    println!(
        "Original: {:?}, Updated: {:?}",
        another_tuple, updated_another_tuple
    );
    // task 5..end..

    // task 6 Using usize for the size of a collection
     let numbers = vec![2, 5, 8, 9, 12, 15];
    let result = count_even_numbers(&numbers);
    println!("Even numbers: {}, Length of vector: {}", result.0, result.1);
    // task 6..end..

    // task 7 Casting with `as`
     let my_numbers = vec![10, 20, 30, 40, 50];
    let result = sum_and_cast(my_numbers);
    println!(
        "Sum as u32: {}, Sum as usize: {}",
        (10u32 + 20 + 30 + 40 + 50),
        result
    );
    let empty_vec: Vec<u32> = vec![];
    let result_empty = sum_and_cast(empty_vec);
    println!("Sum as u32: 0, Sum as usize: {}", result_empty);
    // task 7..end..

    // task 8 Putting datatypes into options and putting options into collections
     let vector = vec![-2, 1,2,3,4,5,6,7,8];
    let result = from_index(vector, 7);

    println!("{:?}", result);

    let vector = vec![7,8];
    let result = from_index(vector, 2);

    println!("{:?}", result);
    // task 8..end..
}
