fn main() {
    // task 1 Introduction to floating-point values.
    let x = 11;
    let result = div_2(x);
    println!("{}", result);
    // task 1..end..

    // task 2 Floating points values must be written with decimals
     let x = 5.0;
    accept(x);
    // task 2..end..

    // task 3 Floating points are copy types
    let x = 1.0;
    let y = x;
    
    println!("{} {}", x, y);
    // task 3..end..

    // task 4 Safely casting from integer to float requires that the integer be strictly smaller
     let x = 3;
    let y = 4;
    
    let result = div(x, y);
    println!("{}", result);
    // task 4..end..

    // task 5 Floating point min and max values
     let result1 = f32_range();
    let result2 = f64_range();
    
    println!("{:?}", result1);
    println!("{:?}", result2);
    // task 5..end..

    // task 6 strict equality of floats danger
    let x = 0.1;
    let y = 0.2;
    let z = 0.3;
    
    println!("{:?}", sum_eq(x, y, z));
    // task 6..end..

    // task 7 Epsilon comparison
    let x = 9.9;
    let y = 10.0;
    let pct_delta = 0.012;
    let result = approx_equal(x, y, pct_delta);
    println!("{}", result);
    // task 7..end..

    // task 8 .parse() with floats
    let v = vec!["1.0", "hello", "0.1"];
    let result = convert_vec(v);
    println!("{:?}", result);
    // task 8..end..

    // task 9 sqrt
    let x = f32::sqrt(-9.0);
    
    let result = f32::sqrt(x);
    println!("{:?}", result);
    // task 9..end..

    // task 10 Pythagorean theorem
    let x = 3.0;
    let y = 4.0;
    let result = pythagorean_theorem(x, y);
    println!("{}", result);
    // task 10..end..

    // task 11 Percent Calculator
    let x = 3;
    let y = 4;
    let result = percent_calculator(x, y);
    println!("{}", result);
    // task 11..end..

    // task 12 Area of a circle
     let r = 9.0;
    let result = area(r);
    println!("{}", result);
    // task 12..end..

    // task 13 Geometric mean
     let x = 9.0;
    let y = 10.0;
    let result = geometric_mean(x, y);
    println!("{}", result);
    // task 13..end..

    // task 14 Convert &Vec<&i16> to Vec<f32>
    let v = Vec::from([&16, &17, &18, &19, &20]);
    
    let result = convert(&v);
    println!("{:?}", result);
    // task 14..end..
}

// task 1 Introduction to floating-point values
pub fn div_2(x: i32) -> f64 {
    return f64::from(x) / 2.0;
}
// task 1..end..

// task 2 Floating points values must be written with decimals
pub fn accept(_x: f32) {
    
}
// task 2..end..

// task 4 Safely casting from integer to float requires that the integer be strictly smaller
pub fn div(x: u16, y: u32) -> f32 {
    return f32::from(x)/(y as f32)
}
// task 4..end..

// task 5 Floating point min and max values
pub fn f32_range() -> (f32, f32) {
    return (f32::MIN, f32::MAX)
}

pub fn f64_range() -> (f64, f64) {
    return (f64::MIN, f64::MAX)
}
// task 5..end..

// task 6 strict equality of floats danger
pub fn sum_eq(x: f64, y: f64, z: f64) -> bool {
    if z == x+y{
        return true;
    }else{
        false
    }

}
// task 6..end..

// task 7 Epsilon comparison
pub fn approx_equal(x: f32, y: f32, pct_delta: f32) -> bool {
    let abs_x = f32::abs(x);
    let abs_y = f32::abs(y);
    let min_val = if abs_x < abs_y {
        abs_x
    }else {
        abs_y
    };
    let diff = f32::abs(abs_x - abs_y);

    diff / min_val <= pct_delta
  
}
// task 7..end..

// task 8 .parse() with floats
pub fn convert_vec(v: Vec<&str>) -> Vec<f32> {
    v.into_iter().map(|x| x.parse::<f32>()).filter(|x| x.is_ok()).map(|x| x.unwrap()).collect()
}
// task 8..end..

// task 10 Pythagorean theorem
pub fn pythagorean_theorem(a: f32, b:f32) -> f32 {
     f32::sqrt(a*a+ b*b)
}
// task 10..end..

// task 11 Percent Calculator
pub fn percent_calculator(x: u32, y: u32) -> f64 {
    f64::from(x)/ f64::from(y)
}
// task 11..end..

// task 12 Area of a circle
pub fn area(x: f32) -> f32 {
    x * x * std::f32::consts::PI
    
}
// task 12..end..

// task 13 Geometric mean
pub fn geometric_mean(x:f32, y:f32)->f32 {
    f32::sqrt(x*y)
}
// task 13..end..

// task 14 Convert &Vec<&i16> to Vec<f32>
pub fn convert(v: &Vec<&i16>) -> Vec<f32> {
    v.into_iter().map(|&&x| f32::from(x)).collect()
}
// task 14..end..