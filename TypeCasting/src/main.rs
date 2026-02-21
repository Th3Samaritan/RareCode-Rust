fn main() {
    // task 1 Rust If Syntax
	let result = max(50,22);
	println!("{}", result);
    // task 1..end..

    // task  2 Absolute Value
    let result = absolute_value(-50);
	println!("{}", result);
    // task 2..end..

    // task 3 Absolute Value with Type Conversion
    let result = absolute_value(-50);
	println!("{}", result);
    // task 3..end..

    // task 4 Integer Type Conversion with Underflow
    let result = foo(-1); 
	println!("{}", result);
    // task 4..end..

    // task 5 Find Element Index with Returns
   let v = vec![1,2,3];
	let k = 4;
	let result = find(&v, k);
	println!("{}", result);

	let k = 2;
	let result = find(&v, k);
	println!("{}", result);
    // task 5..end..

    // task 6
       let result_true = can_downcast(255);
    println!("{}", result_true);
    let result_false = can_downcast(256);
    println!("{}", result_false);
    // task 6..end..

    // task 7
     let result_true = can_downcast(127);
    println!("{}", result_true);
    let result_false = can_downcast(128);
    println!("{}", result_false);
    let result_true_neg = can_downcast(-128);
    println!("{}", result_true_neg);
    let result_false_neg = can_downcast(-129);
    println!("{}", result_false_neg)
    // task 7..end..

    // task 8
    let x = 3;
	let y = 5;
	let z = 4;
	
	let result = max_of_three(x, y, z);
	println!("{}", result);
    // task 8..end..

    // task 9
    let v_ok = vec![10, -20, 127, -128, 0];
	let v_bad_high = vec![10, 128, 50];
	let v_bad_low = vec![-129, 0, 100];

	println!("{:?} -> {}", v_ok, vector_can_be_downcasted(&v_ok));
	println!("{:?} -> {}", v_bad_high, vector_can_be_downcasted(&v_bad_high));
	println!("{:?} -> {}", v_bad_low, vector_can_be_downcasted(&v_bad_low));
	
	let empty_vec: Vec<i32> = vec![];
	println!("{:?} -> {}", empty_vec, vector_can_be_downcasted(&empty_vec));
    // task 9..end..

    // task 10
     println!("100 clamped: {}", clamp(100));
	println!("150 clamped: {}", clamp(150));
	println!("-100 clamped: {}", clamp(-100));
	println!("-200 clamped: {}", clamp(-200));
	println!("127 clamped: {}", clamp(127));
	println!("-128 clamped: {}", clamp(-128));
    // task 10..end..
}
// task 1 Rust If Syntax
pub fn max(x: i32, y: i32) -> i32 {
	let m = if x > y {
		x 
	} else {
		y 
	m
} 
}
// task 1..end..

// task 2 Absolute Value
pub fn absolute_value(x: i32) -> i32 {
	let ans = if x < 0 {
		-x
    } else {
		x
	};
	ans
} 
// task 2..end..

//  task 3 Absolute Value with Type Conversion
pub fn absolute_value(x: i32) -> u32 {
	let ans = if x < 0 {
		x * -1
	} else {
		x
	};
    ans as u32
} 
// task 3..end..

// task 4 Integer Type Conversion with Underflow
pub fn foo(x: i32) -> u32 {
    let ans = x as u32;
    ans
} 
// task 4..end..

// task 5 Find Element Index with Returns
pub fn find(v: &Vec<i32>, k: i32) -> i32 {
	for i in 0..v.len() {
	    if k == v[i] {
            let j = i as i32;
	        return j;
	    }
	}
	-1
} 
// task 5..end..

// task 6 Check if Integer Can Be Downcast
pub fn can_downcast(x: u32) -> bool {
	if x <= u8::MAX as u32 { 
		true
	} else {
		false
	}
} 
// task 6..end..

// task 7 Check if Signed Integer Can Be Downcast
pub fn can_downcast(x: i32) -> bool {
	if x <= i8::MAX as i32 && x >= i8::MIN as i32{ // cast here
		true
	} else {
		false
	}
} 
// task 7..end..

// task 8 Maximum of Three Numbers
pub fn max_of_three(x: i32, y: i32, z: i32) -> i32 {
    let m = if x>=y && x>=z {
        x
    }else if y>=x &&y>=z{
        y
    }
    else{
        z
    };
m
} 
// task 8..end..

// task 9 Check Vector Elements for i8 Range
pub fn vector_can_be_downcasted(v: &Vec<i32>) -> bool {
	for i in 0..v.len()  {
        if v[i] < i8::MIN as i32 || v[i] > i8::MAX as i32{
            return false;
        }
    }
    true
} 
// task 9..end..

// task 10 Clamp Integer Values
pub fn clamp(x: i32) -> i8 {
    if x > i8::MAX as i32 {
        i8::MAX
    } else if x< i8::MIN as i32{
        i8::MIN
    }
    else{
        x as i8
    }
} 
// task 10..end..