fn main() {
    // task 1
    let result = div_zero();
    println!("{}", result);
    // task 1..end..

    // task 2 NAN is not equal to NAN
    let f1 = f32::NAN;
    let f2 = f32::NAN;
    let result = float_compare(f1, f2);
    println!("{}", result);
    // task 2..end..

    // task 3 Comparing NAN with > or ≥
     let result = compare_nan(1.0);
    println!("{:?}", result);
    // task 3..end..

    // task 4 NAN propagates through arithmetic
    let x = 1.0;
    let result = nan_add(x);
    println!("{}", result);
    // task 4..end..

    // task 5 Detecting NaN with is_nan()
    let x = 0.0 / 0.0;
    println!("{}", check_nan(x));
    // task 5..end..

    // task 6 Filtering NAN and Infinity
    let values = vec![1.0, f32::NAN, f32::INFINITY, -2.5, f32::NEG_INFINITY, 3.3];
    let result = filter_finite(values);
    println!("{:?}", result);
    // task 6..end..

    // task 7 Average Ignoring NAN
    let values1 = [1.0, 2.0, 3.0];
    let values2 = [1.0, f32::NAN, 3.0];
    let values3 = [f32::NAN, f32::INFINITY];

    println!("{:?}", average_ignore_nan(&values1));
    println!("{:?}", average_ignore_nan(&values2));
    println!("{:?}", average_ignore_nan(&values3));
    // task 7..end..
}

// task 1
pub fn div_zero() -> f32 {
    0.0/0.0
}
// task 1..end..

// task 2 NAN is not equal to NAN
pub fn float_compare(f1: f32, f2: f32) -> bool {
    f1==f2
}
// task 2..end..

// task 3 Comparing NAN with > or ≥
pub fn compare_nan(x: f32) -> (bool, bool, bool, bool) {
    let nan = f32::NAN;
    (nan > x, nan < x, nan >= x, nan <= x)
}
// task 3..end..

// task 4 NAN propagates through arithmetic
pub fn nan_add(x: f32) -> f32 {
    let nan = f32::NAN;
    x + nan
}
// task 4..end..

// task 5 Detecting NaN with is_nan()
pub fn check_nan(x: f32) -> bool {
    if x.is_nan(){
        return true;
    }else {
        false
    }
}
// task 5..end..

// task 6 Filtering NAN and Infinity
pub fn filter_finite(v: Vec<f32>) -> Vec<f32> {
    v.into_iter().filter(|x| !x.is_nan() && !x.is_infinite()).collect()
}
// task 6..end..

// task 7 Average Ignoring NAN
pub fn average_ignore_nan(values: &[f32]) -> Option<f32> {
    let mut sum = 0.0;
    let mut count = 0;
    for &v in values{
    if v.is_finite(){
        sum += v;
        count += 1;
        }
    }
    if count == 0 {
        None
    } else {
        Some(sum /count as f32)
    }

}
// task 7..end..