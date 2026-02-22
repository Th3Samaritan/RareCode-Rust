fn main() {
    // task 1 Option Type Printing
    let v = vec![1,2,3];
    
    let result = v.get(6);
    
    println!("{:?}", result);
    // task 1..end..

    // task 2 is_none() method
     let v = vec![1,2,3];
    
    let result = v.get(7); 
    
    if result.is_none() {
        println!("{}", "No value");
    }

    if !result.is_none() {
        println!("Value: {:?}", result);
    }
    // task 2..end..

    // task 3 Option Unwrapping: Safe Vector Access
     let v = vec![1,2,3]; 
    
    let result = v.get(0);
    
    if !result.is_none() {
	    let sum = result.unwrap() + 1; 
        println!("{}", sum);
    }
    // task 3..end..

    // task 4
     let x = 10;
    let y = 2;
    
    let result = div(x,y);
    
    if !result.is_none() {
        println!("{}", result.unwrap());
    } else {
        println!("{}", "divide by zero");
    }

    let y_zero = 0;
    let result_zero = div(x, y_zero);
    if !result_zero.is_none() {
        println!("{}", result_zero.unwrap());
    } else {
        println!("{}", "divide by zero");
    }
    // task 4..end..

    // task 5 Working with Option Values
    let a = Some(true);
	let b = Some(vec![1,2,3]);
	let c = Some(7);
    println!( "{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    // task 5..end..

    // task 6
     println!("1 -> {:?}", to_bool(1));
    println!("0 -> {:?}", to_bool(0));
    println!("5 -> {:?}", to_bool(5));
    println!("-1 -> {:?}", to_bool(-1));
    // task 6..end..

    // task 7
    let x_ok = 12345678u64;
	let result_ok = from_u64_to_u32(x_ok);
	println!("{} -> {:?}", x_ok, result_ok);

	let x_big = u32::MAX as u64 + 1;
	let result_big = from_u64_to_u32(x_big);
	println!("{} -> {:?}", x_big, result_big);
    // task 7

    // task 8 Sum Present Values in Option Vector
      let values1 = vec![Some(2), None, Some(5), Some(3), None];
    let result1 = sum_present(values1);
    println!("Result 1: {:?}", result1); 
    let values2 = vec![None, None, None];
    let result2 = sum_present(values2);
    println!("Result 2: {:?}", result2); 

    let values3 = vec![Some(10), Some(20), Some(-5)];
    let result3 = sum_present(values3);
    println!("Result 3: {:?}", result3); 

    let values4: Vec<Option<i32>> = vec![];
    let result4 = sum_present(values4);
    println!("Result 4: {:?}", result4);
    // task 8..end..

    // task 10 Safe Signed Integer Conversion with Option Types
     println!("100 -> {:?}", safe_i32_to_i8(100));
    println!("127 -> {:?}", safe_i32_to_i8(127));
    println!("128 -> {:?}", safe_i32_to_i8(128));
    println!("-128 -> {:?}", safe_i32_to_i8(-128));
    println!("-129 -> {:?}", safe_i32_to_i8(-129));
    // task 10

    // task 11 Vector Modification with Option Types
     let v = vec![1,2,3];
    let result_ok = double_at(&v, 1);
    println!("Double index 1: {:?}", result_ok);

    let result_none = double_at(&v, 5);
     println!("Double index 5: {:?}", result_none);

     let result_empty = double_at(&Vec::<i32>::new(), 0);
     println!("Double index 0 (empty): {:?}", result_empty);
     // task 11

     // task 12
     let x = 48;
    let y = 18;
    let result = gcd(x, y);
    println!("{}", result);
    // task 12..end..

    // task 13 Combining Option Values with Arithmetic Operations
    let a = Some(5);
    let b = Some(10);
    let c: Option<i32> = None;

    println!("{:?} + {:?} = {:?}", a, b, add_options(a, b));
    println!("{:?} + {:?} = {:?}", a, c, add_options(a, c));
    println!("{:?} + {:?} = {:?}", c, b, add_options(c, b));
    println!("{:?} + {:?} = {:?}", c, c, add_options(c, c));
    // task 13..end..

    // task 14 Division with Options
     let a = Some(10);
    let b = Some(2);
    let b_zero = Some(0);
    let c: Option<i32> = None;

    println!("{:?} / {:?} = {:?}", a, b, div_options(a, b));
    println!("{:?} / {:?} = {:?}", a, b_zero, div_options(a, b_zero));
    println!("{:?} / {:?} = {:?}", a, c, div_options(a, c));
    println!("{:?} / {:?} = {:?}", c, b, div_options(c, b));
    println!("{:?} / {:?} = {:?}", c, c, div_options(c, c));
    // task 14..end..
}

// task 4 Safe Division with Option Types
pub fn div(n: i32, d: i32) -> Option<i32> {
    if d == 0 {
        None
    } else {
        Some(n / d)
    }
} 
// task 4..end..

// task 6 Integer to Boolean Conversion with Option Types
pub fn to_bool(n: i32)->Option<bool>{
    if n == 1 {
        Some(true)
    } else if n == 0 {
        Some(false)
    } else {
        None
    }
}
// task 6..end..

// task 7 Safe Integer Type Conversion with Option Types
pub fn from_u64_to_u32(x: u64)->Option<u32>{
    if x <= u32::MAX as u64 {
        Some(x as u32)
    }else {
        None
    }
} 
// task 7

// task 8
pub fn sum_present(values: Vec<Option<i32>>) -> i32 {
    let mut total = 0;
    for i in values.into_iter(){
    if i.is_some(){
        total = total + i.unwrap();
    }
    }
    total
}
// task 8..end..

// task 10 Safe Signed Integer Conversion with Option Types
pub fn safe_i32_to_i8(x: i32)->Option<i8>{
    if x >= i8::MIN as i32 && x <= i8::MAX as i32 {
        Some(x as i8)
    } else {
        None
    }
}
// task 10

// task 11 Vector Modification with Option Types
pub fn double_at(v: &Vec<i32>, idx:usize)->Option<Vec<i32>>{
    let mut x = v.clone();
        if idx <  v.len(){
            x[idx] = x[idx] * 2; 
            Some(x)

        } else {
            None
        }
}
// task 11..end..

// task 12 Greatest Common Divisor
pub fn gcd(x: u32, y: u32) -> u32 {
    let mut min = x;
    if y < x {
        min = y;
    }

    for i in 0..min{
        let current = min - i;

        if x % current == 0 && y % current ==0 {
            return current;
        }
    } 1
}
    // task 12..end..

    // task 13 Combining Option Values with Arithmetic Operations
    pub fn add_options(a:Option<i32>, b:Option<i32>)->Option<i32>{
    if a.is_none() || b.is_none(){
        return None;
    }
    Some(a.unwrap() + b.unwrap())
}
// task 13..end..

// task 14 Division with Options
pub fn div_options(a:Option<i32>, b:Option<i32>)->Option<i32>{
    if a.is_none() || b.is_none() {
        return None;
    }
    let val_a = a.unwrap();
    let val_b = b.unwrap();
    if val_b == 0 {
        return None;
    }
    Some(val_a/ val_b)
} 
// task 14..end..