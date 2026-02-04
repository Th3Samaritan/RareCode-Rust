// task 1 Mutable variables can be completely re-assigned
fn main() {
	let mut v = vec![1,2,3];
	println!("{:?}", &v);
	v = vec![4,5,6];
	println!("{:?}", &v);
// task 1..end..

// task 2 Taking ownership of a non-mutable collection
	let w = vec![4,5,6];
    
    v = w; 
    
    v.push(7);
	 println!("{:?}", v);
// task 2..end..

// task 3 Re-assigning causes an ownership change
v = w.clone();
    
    v.push(7);
    
    println!("{:?}", w);

	// task 3..end..

// task 4 Re-assignment must be to the same type
 let s = HashSet::from([&4, &5, &6]);
    
    v = s.into_iter().copied().collect();
    println!("v is {:?}", v);
// task 4..end..


}

// task 5 Exercise: reassign to longest

pub fn assign_to_longest(mut v: Vec<i32>, a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // your code here
    if v.len() >= a.len() || v.len() >= b.len(){
        return v;
    }
    if a.len() > v.len() && a.len()>=b.len() {
        v=a;
    } else if b.len() > v.len() && b.len() > a.len(){
        v=b;
    }
    v
}
// task 5..end..

// task 6 very evil thing most annoying, Exercise: reassign to the largest sum

pub fn assign_to_largest_sum(mut v: Vec<i32>, a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // your code here
let sum_a: i32 = a.iter().sum();
let sum_b: i32 = b.iter().sum();
if sum_a >= sum_b {
    v = a;
} else {
    v = b;
    };
    v
}
// task 6..end..

//task 7 assitant evil very tricky exercise, Cannot reassign if a reference exists
fn main() {
    
    let mut v: Vec<i32> = vec![];
       v = vec![3,2,1];
       println!("{:?}", &v);
    let r = &v;
 
    
    println!("{:?}", r);
    
}
// task 7..end..