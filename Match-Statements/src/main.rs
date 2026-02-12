use std::collections::HashMap;
fn main() {
    // task 1 introduction to match statements
    let a = 2;

    match a {
        0 => println!("zero"),
        1 => println!("one"),
        _ => println!("neither"),
    };
    // task 1..end..

    // task 2 Match statement as an expression
     let written_form = match a {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "something else",
    };
    
    println!("{}", written_form);
    // task 2..end..

    // task 3 Match Syntax
     let string_slice = "true";

    let option_bool = match string_slice {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    };

    println!("{:?}", option_bool);
    // task 3..end..

    // task 4 Using match to destructure
     let t = (0, 1);
    let result = increment_if_not_zero(t);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Exercise: Tuple Mirror
         let t = (0, 0, 2);
    let result = tuple_mirror(t);
    println!("{:?}", result);
    // task 5..end..

    // task 6 Destructuring an Option
    let opt = Some(0);

    match opt {
        Some(0) => println!("zero"),
        Some(_) => println!("non-zero"),
        None => println!("empty"),
    }
    // task 6..end..

    // task 7 Match inside a closure
      let v = vec![Some(1), None, None, Some(-1)];
    let result = abs_inside_value(&v);
    println!("{:?}", result);
    // task 7..end..

    // task 8 IncInside
      let mut v = vec![Some(1), None, None, Some(-1)];
    inc_inside(&mut v);
    println!("{:?}", v);
    // task 8..end..

    // task 9 HashMap get or zero
        let map = HashMap::from([(1, 10), (2, 20), (3, 90)]);
    
    let value = get_or_zero(&map, 1);
    println!("Value for key 1: {}", value); // Should print 10

    let value = get_or_zero(&map, 4);
    println!("Value for key 4: {}", value); // Should print 
    // task 9..end..
}

// task 4 Using match to destructure
pub fn increment_if_not_zero(t: (u32, u32)) -> (u32, u32) {
    match t {
        (0, 0) => (0, 0),
        (0, y) => (0, y + 1),
        (x, 0) => (x + 1, 0),
        (x, y) => (x + 1, y + 1),
    }
}
// task 4..end..

// task 5 Exercise: Tuple Mirror
pub fn tuple_mirror(t: (i32, i32, i32)) -> (i32, i32, i32) {
   match t {
    (a, 0, b) => (b, 0, a),
    (a, b, c) => (a, b, c),
   }
}
// task 5..end..

// task 7 Match inside a closure
pub fn abs_inside_value(v: &[Option<i32>]) -> Vec<Option<i32>> {
    v.iter().map(|&x| {
        match x {
            Some(val)  => {
              if val < 0{
                Some(-val)
              }else {
                Some(val)
              }
            }
            _ => x,
        }
    }).collect()
}
// task 7..end..

// task 8 IncInside
pub fn inc_inside(v: &mut Vec<Option<i32>>) {
     for i in v {
        match i {
            Some(val) => *i = Some(*val + 1),
            None => {}
        }
     }
}
// task 8..end..

// task 9
pub fn get_or_zero(map: &HashMap<i32, i32>, key: i32) -> i32 {
    match map.get(&key) {
        Some(&value) => value,
        None => 0,
    }
}
// task 9..end..