// task 1
fn main() {
    let mut v = vec![1, 2, 3];

    for (_i, e) in v.iter_mut().enumerate() {
        //*i = *i + 1;
        *e += *e;
    }
    println!("{:?}", v);
}

// task 1..end..

// task 2 Example: Add index to each

pub fn add_to_index(v: &mut Vec<i32>) {
    for (i, e) in v.iter_mut().enumerate(){
        *e = *e + (i as i32);
    }
}
// task 2..end..

// task 3 Exercise: subtract index from each
pub fn sub_by_index(v: &mut Vec<i32>) {
     for (i, e) in v.iter_mut().enumerate(){
        *e = *e - (i as i32);
    }
}
// task 3..end..