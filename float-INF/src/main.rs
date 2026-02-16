fn main() {
   //task 1 Overflow goes to infinity
    let max_f64 = f64::MAX;
    let overflow_to_inf = max_f64 * 2.0;
    println!("{}", overflow_to_inf);
// task 1..end..

// task 2 Underflow to negative infinity
 let x = f32::MAX;
    let result = f(x);
    println!("{}", result);
    // task 2..end..

    // task 3 (1 / 0 vs -1 / 0)
    let x1 = 1.0;
    let x2 = -1.0;
    
    let result1 = div_0(x1);
    let result2 = div_0(x2);
    
    println!("{}", result1);
    println!("{}", result2);
    // task 3..end..

    // task 4 Multiply by infinity
     let x1 = f32::INFINITY;
    let x2 = 3.0;
    
    println!("{}", mul_infinity(x1));
    println!("{}", mul_infinity(x2));
    // task 4..end..

    // task 5 a > b does not always mean a × k > b × k
     let a = 4.0;
    let b = 3.0;
    
    println!("{}", property_holds(a, b, 6.0));
    println!("{}", property_holds(a, b, f32::MAX));
    // task 5..end..

    // task 6 Hadamard Product
    let a1 = [1.0, 2.0, 3.0];
    let a2 = [1.0, 2.0, f32::INFINITY];
    
    let result = element_wise_mul(&a1, &a2);
    println!("{:?}", result);
    // task 6..end..
}

// task 2 Underflow to negative infinity
pub fn f(x: f32) -> f32 {
    x * -2.0
}
// task 2..end..

// task 3 (1 / 0 vs -1 / 0)
pub fn div_0(x: f32) -> f32 {
        x/0.0
}
// task 3..end..

// task 4 Multiply by infinity
pub fn mul_infinity(x: f32) -> f32 {
    x * f32::INFINITY
}
// task 4..end..

// task 5 a > b does not always mean a × k > b × k
pub fn property_holds(a: f32, b: f32, k: f32) -> bool {
    assert!(a > 0.0);
    assert!(b > 0.0);
    assert!(k > 0.0);
    
    if a > b {
        return a * k > b * k
    }
    
    true
}
// task 5..end..

// task 6 Hadamard Product
pub fn element_wise_mul(a: &[f32], b: &[f32]) -> Vec<Option<f32>> {
    let mut result = Vec::new();

    for (i, &val_a) in a.iter().enumerate() {
        let val_b = b[i];
        let product = val_a * val_b;
     if val_a.is_finite() && val_b.is_finite() && product.is_finite() {
            result.push(Some(product));
        } else {
            result.push(None);
        }
    }
    result
}
// task 6..end..