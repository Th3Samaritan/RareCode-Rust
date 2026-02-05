// task 1 for e in &mut v as an equivalent to for e in v.iter_mut()
fn main() {
	let mut v = vec![1,2,3];
	
	for e in &mut v {
		*e = *e * *e;
	}
	
	println!("all values squared {:?}", v);
    // task 1..end..
    
    // task 2 Exercise: Make the code idiomatic 1
    	let mut v = vec![7,8,9];
	for e in &mut v {
	    *e = inc_if_odd(e);
	}
	println!("{:?}", v);

    // task 2..end..

    // task 3 Exercise: Make the code idiomatic 2
    let v = vec![1,2,3,4];
    let result = running_sum(v);
    println!("{:?}", result);
}

// task 2 Exercise: Make the code idiomatic 1

pub fn inc_if_odd(e: &i32) -> i32 {
    if e % 2 == 1 {
        return e + 1;
    }
    *e
}
// task 2..end..

// task 3 Exercise: Make the code idiomatic 2

pub fn running_sum(mut v: Vec<i32>) -> Vec<i32> {

    let mut prev = 0;
    
    for e in &mut v {
        *e = *e + prev;
        prev = *e;
    }
    
    v
}
// task 3..end..

// task 4 Exercise: Clamp and Cut
fn main() {
	let v = vec![72057594037927936, 281474976710656, 131072];
	
	let result = clamp_and_cut(v);
	println!("{:?}", result); // [2147483647, 2147483647, 65536]
}

pub fn clamp_and_cut(mut v: Vec<u64>) -> Vec<u64> {
		for i in &mut v{
            if *i > u32::MAX as u64{
                *i = u32::MAX as u64;
            }
        }
        for i in &mut v{
            *i = *i/2;
        }
        v
}
// task 4..end..

// task 6 for e in &mut v when v is already &mut part 2

pub fn square_each(v: &mut Vec<i32>) {
    
    for e in v.iter_mut() {
        *e = *e * *e;
    }
}
// task 6..end..

// task 7 Exercise: Square and Increment

pub fn square_and_inc(v: &mut Vec<i32>) {
    
    for e in v.iter_mut() {
        *e = *e * *e;
    }
    
    for e in v.iter_mut() {
        *e = *e + 1;
    }
}
// task 7..end..