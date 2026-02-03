// task 1 Mutable References
fn main() {
	let mut v = vec![1,2,3];
	
	append_zero(& mut v);
    // task 2 Mutable References do not take onwnership
    append_zero(&mut v);
	append_zero(&mut v);
	append_zero(&mut v);
    append_zero(&mut v);
    // task 2..end.. Print the vector

    //task 3 Append Length
	append_len(&mut v);
    // task 3..end.. Print the vector

    // task 8 create a mutable reference as a variable
    let mr = &mut v; 
	
	accept(mr);
    // task 8..end.. Print the vector
	println!("{:?}", v);
}

pub fn append_zero(v: &mut Vec<i32>) {
    v.push(0);
}

// task 3 Append Length
pub fn append_len(v: &mut Vec<i32>) {
    let y = v.len();
    v.push(y as i32);
    }

//task 4 Mutation is not mandatory
pub fn sum(v: &Vec<i32>) -> i32 {
    // compute the sum here without mutating
   v.iter().sum()
}

// task 5 Rust doesnt check if the function actually mutates
// no actual mutation happens here
pub fn max(v: &mut Vec<i32>) -> Option<i32> {
    v.iter().max().copied()
}

// task 6 Increment all

pub fn inc_all(v: &mut Vec<i32>) { 
    for i in 0..v.len(){
       v[i] = v[i]+1;
    }
}

// task 7 unwrap increment wrap
pub fn unwrap_inc_wrap(v: &mut Vec<Option<i32>>) {
for i in 0..v.len(){
    if v[i].is_some(){
	let val =v[i].unwrap();
    v[i] = Some(val + 1);
        }
    
    }
}