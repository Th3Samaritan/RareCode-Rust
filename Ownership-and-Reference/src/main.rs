fn main() {
// task 1 Understanding Rust Ownership
	let v = vec![1,2,3];
	do_nothing(v); 
// task 1..end..

// task 2 Understanding Ownership Through Variable Assignment
let v = vec![1,2,3];
	let w = v;
	println!("{:?}", w);
// task 2..end..

// task 3 Ownership practice
let v = vec![4,5,6];
	let u = vec![7,8,9];
	
	let a = v;
	let b = u;
    println!("{:?}", a);
	println!("{:?}", b);
    // task 3..end..

    // task 4
    let v = vec![4,5,6];
	
	let v_sum = vec_sum(v); 
	
	println!("{}", v_sum);
    // task 4..end..

    // task 5
    let v = vec![1,2,3];
	let sum = vec_sum(v); 
	
	println!("{:?}", v);
	println!("{}", sum);
    // task 5..end..

    // task 6
    let x = vec![1,2,3];
	let y = &x;
	
	println!("{:?}", x);
	println!("{:?}", y);
    println!("{}", vec_sum(&x));
    println!("{}", vec_sum(y));
    // task 6..end..

    // task 7
    let v1 = vec![1,2,3];
	let v2 = vec![1,1,2];
	
	let result = elementwise_sum(&v1, &v2);
	println!("{:?}", v1);
	println!("{:?}", v2);
	println!("{:?}", result);
    // task 7..end..

    // task 8
    let v = vec![1,1,5];
	let cond1 = all_elements_less_than_k(&v, 6); 
	let cond2 = sum_greater_than_s(&v, 6); 
	println!("{}", cond1 && cond2);
    // task 8..end..

    // task 9 Filter Numbers Greater Than or Equal to K
    let v = vec![4,8,14];
	let k = 8;
	let result = filter_lt_k(&v, k);
	println!("{:?}", result);
    // task 9..end..

}
//  task 1 Understanding Rust Ownership
pub fn do_nothing(v: Vec<i32>) -> Vec<i32> {
	v
} 
// task 1..end..

// task 4 Ownership practice 2
pub fn vec_sum(v: Vec<i32>) -> i32 {
	let mut sum = 0;
	
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
	sum
} 
// task 4..end..

// task 5 Vector Sum with References
pub fn vec_sum(v: Vec<i32>) -> i32 {
	let mut sum = 0;
	
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
	sum
} 
// task 5..end..

// task 6 Variable assignment to a reference
pub fn vec_sum(v: &Vec<i32>) -> i32 {
	let mut sum = 0;
	
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
	sum
} 
// task 6..end..

// task 7 Element-wise Sum
pub fn elementwise_sum(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    let mut y = Vec::new();
    for i in 0..v1.len(){
	if v1.len() == v2.len() {
       y.push (v1[i] + v2 [i])
    }
} y
}
// task 7..end..

// task 8 Reusing vectors
pub fn all_elements_less_than_k (v: &Vec<i32>, k: i32)->bool {
    for i in 0..v.len(){
        if v[i] >= k {
            return false;
        }
    }true
}
pub fn sum_greater_than_s(v: &Vec<i32>, s: i32) -> bool {
    let mut sum = 0;
    for i in 0..v.len(){
        sum = sum + v[i];
        if sum > s {
            return true;
        }
    }false
}
// task 8..end..

// task 9 Filter Numbers Greater Than or Equal to K
pub fn filter_lt_k(v: &Vec<i32>, k:i32) ->Vec<i32> {
    let mut y = vec![];
    for i in 0..v.len(){
        if v[i] >= k {
            y.push(v[i])
        }
    }y
}
// task 9..end..