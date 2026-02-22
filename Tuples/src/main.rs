//task 11
pub fn accept_tuple(_t: &(i32, bool, Vec<i32>)) {

} 
// task 11..end..

// task 5 Tuple Return Value for Min and Max
 pub fn min_and_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
// task 5..end..

// task 6
pub fn append_total(input: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = vec![];
    let mut sum0 = 0;
    let mut sum1 = 0;
     for i in 0..input.len(){
        output.push(input[i]);
        sum0 = sum0 + input[i].0;
        sum1 = sum1 + input[i].1;

     }
     output.push((sum0, sum1));
		
    output
}
// task 6..end..

// task 8
pub fn append_total(input: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = vec![];
    let mut sum0 = 0;
    let mut sum1 = 0;
    
    for (a, b) in input {
        output.push((a, b));
        sum0 = sum0 + a;
        sum1 = sum1 + b;
    }

    output.push((sum0, sum1));
    output
}
// task 8..end..

// task 12
pub fn update_tuple_elements(t: &(Vec<i32>, bool, i32)) -> (Vec<i32>, bool, i32) {
    let mut t2 = t.0.clone();
    t2.push(1);
    let second = !t.1;
    let third = t.2 + 1;
    (t2, second, third)
}
// task 12..end..

// task 13
pub fn clone_filter_update(input: &Vec<(i32, bool)>) -> Vec<(i32, bool)> {
    let cloned = input.clone();
    let mut result = Vec::new();
        for (num, _status) in cloned {
            if num % 2 == 0 {
                result.push((num, true));
            }
        }

    result
}

// task 13..end..

fn main() {
    // task 1 Tuple Initialization and Display
    let x: (i32, bool) = (2, true);
    let y: (u32, i32, bool) = (2, -5, false);
    let z: (Vec<i32>, i32, bool) = (vec![-5, 2, 3], 8, true);
    let p: (i32, i32) = (2, 3);
    let unit: () = (); 

    println!("{:?}, {:?}, {:?}, {:?}, {:?}", x, y, z, p, unit);
    // task 1..end..

    // task 2 Tuple Ownership and Assignment
     let x = (3, vec![1, 2, 3]);
    let y = x;
    println!("{:?}", y); 
    // task 2..end..

    // task 3 Tuple Copy Trait Behavior
      let x = (3, 1, true);
    let y = x;
    println!("{:?}", y);
    // task 3..end..

    // task 4 Tuple Element Access
     let pair: (bool, u32, i32) = (true, 2025, -5);
    println!("Status: {}, Year: {}, GPA: {}", pair.0, pair.1, pair.2 );
    // task 4..end..

    // task 5 Tuple Return Value for Min and Max
   let result = min_and_max(5, 3);
    println!("Min: {}, Max: {}", result.0, result.1);
    let result2 = min_and_max(-5, 7);
    println!("Min: {}, Max: {}", result2.0, result2.1);
    let result3 = min_and_max(100, -100);
    println!("Min: {}, Max: {}", result3.0, result3.1)
    // task 5..end..

    // task 6 Vector of Tuples with Totals
    let data = vec![(1, 2), (3, 4), (5, 6), (4,8), (6,9)];
    let result = append_total(data);
    println!("{:?}", result); 
    // task 6..end..

    // task 8 Improved Vector of Tuples with Totals
    let data = vec![(1, 2), (3, 4), (5, 6), (4,8), (6,9)];
    let result = append_total(data);
    println!("{:?}", result);
    // task 8..end..

    // task 9 Mutable Tuple with Vector Modification
    let mut t = (vec![1, 2, 3], 10);
    t.0.push(4); 
    println!("{:?}", t);
    // task 9..end..

    // task 10 Tuple Cloning and Modification
     let t: (i32, i32, i32) = (10, 20, 30);

    let mut new_t = t.clone();

    new_t.0 = new_t.0 + 1;
    new_t.1 = new_t.1 + 1;
    new_t.2 = new_t.2 + 1;

    println!("Original Tuple: {:?}, New Tuple: {:?}", t, new_t);
    // task 10..end..

    // task 11 Tuple References and Function Return
    let data = (99, true, vec![1, 2, 3]);
    accept_tuple(&data); // pass a reference instead
    println!("{:?}", data);
    // task 11..end..

    // task 12 Creating a New Tuple from a Reference
    let t = (vec![10, 20], true, 5);
    
    let updated_t = update_tuple_elements(&t);
    
    println!("Original: {:?}, Updated{:?}",t ,updated_t);
    // task 12..end..

    // task 13 Filtering and Transforming Tuple Vectors
     let input = vec![(2, false), (3, false), (4, false)];
    println!("Original: {:?}, Filtered: {:?}", &input, clone_filter_update(&input) );
    // task 13..end..
}
