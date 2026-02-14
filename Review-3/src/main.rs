// task 7 Exercise: First or Second Half
use std::collections::HashSet;
pub enum Half {
    First,
    Second,
}
// task 7...end..
// task 2 Exercise: Compilation 1
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
}
// task 4 Exercise: Enum match and filter
#[derive(Debug, Clone, Copy)]
pub enum Status {
    Active(u32),
    Expired(u32),
}
// task 4..end..

// task 5
pub enum Parity {
    Even,
    Odd,
}
// task 5..end..

// task 6 Exercise: Compilation 2
pub enum Coin {
    Head,
    Tail,
}
// task 6..end..
// task 1 into
fn main() {
    let sl = "hello";
    let num = 56;
    let a = [1, 2, 3];

    let result_1 = make_string(sl);
    let result_2 = upcast(num);
    let result_3 = make_vector(&a);
		
    println!("{:?} {:?} {:?}", result_1, result_2, result_3);

    // task 2 Exercise: Compilation 1
    let d = Direction::Up;
    let name = to_name(d);
    println!("{:?} {}", d, name);
    
    let d = Direction::Down;
    let name = to_name(d);
    println!("{:?} {}", d, name);
    // task 2..end..

    // task 3
     let msg1 = String::from("rare");
    let msg2 = String::from("code");
    
    let result = earliest_intersection(&msg1, &msg2);
    println!("{:?}", result);
    // task 3..end..

    // task 4 Exercise: Enum match and filter
    let v = Vec::<Status>::from([
        Status::Active(100),
        Status::Expired(60),
        Status::Active(204),
        Status::Expired(59),
    ]);
    let result = only_active(v);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Exercise: String of numbers
    let nums = Vec::<String>::from([
        "12".into(),
        "0".into(),
        "5".into(),
        "muffins".into(),
        "7".into(),
    ]);
    let result = parity(&nums);
    println!("{:?}", result);
    // task 5..end.. 

    // task 6 Exercise: Compilation 2
     let mut coins = vec![Coin::Head, Coin::Tail, Coin::Tail, Coin::Head];

    invert_all(&mut coins);

    println!("{:?}", coins);
    // task 6..end..

    // task 7 Exercise: First or Second Half
    let a = [&1, &2, &3];

    let result = which_half(&a, Half::Second);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Exercise: Compilation 3
    let a = [1, 2, 3];
    let result = product_is_even(&a);
    println!("{}", result);
    // task 8..end..

    // task 9 Exercise: Greater or equal to the sum of all before
    let a = vec![1, 2, 3, 4, 20];
    let result = ge_sum_before(&a);
    println!("{:?}", result);
    // task 9..end..

    // task 10 Exercise: Compilation 4
    let mut a = vec![1, 2, 3, 4];
    sum_of_prev_plus_curr(&mut a);
    println!("{:?}", a);
    // task 10..end..

}

pub fn make_string(sl: &str) -> String {
    sl.into()
}

pub fn upcast(x: i16) -> i32 {
    x.into()
}

pub fn make_vector(sl: &[i32]) -> Vec<i32> {
    sl.into()
}
// task 1..end..

// task 2 Exercise: Compilation 1
fn to_name(d: Direction) -> String {
    match d {
        Direction::Up => "North".into(),
        Direction::Down => "South".into(),
    }
}
// task 2..end..

// task 3

pub fn earliest_intersection(msg1: &str, msg2: &str) -> Option<(usize, usize)> {
    for (i, c1) in msg1.chars().enumerate(){
    for (j, c2) in msg2.chars().enumerate(){
        if c1==c2 {
           return Some((i, j));
        }
    }
    }
    return None
}
// task 3..end..

// task 4 Exercise: Enum match and filter
pub fn only_active(v: Vec<Status>) -> Vec<Status> {
     v.into_iter().filter(|stat| {
        match stat { 
            Status::Active(_) =>true,
             Status::Expired(_) =>false,
            }
        }).collect()
}

// task 4..end..

// task 5 Exercise: String of numbers
pub fn parity(v: &[String]) -> Vec<Option<Parity>> {
    v.iter().map(|s| {
    let res_num = s.parse::<u32>();
    match res_num {
        Ok(n) => {
         if n % 2 == 0 {
            Some(Parity::Even)
        } else {
            Some(Parity::Odd)
             }
                }
         Err(_) => None,
            }
        }).collect()
}
// task 5..end..

// task 6 Exercise: Compilation 2
pub fn invert_all(v: &mut Vec<Coin>) {
    for e in v {
        *e = match e {
            Coin::Head => Coin::Tail,
            Coin::Tail => Coin::Head,
        };
    }
}

// task 6..end..

// task 7 Exercise: First or Second Half

pub fn which_half(sl: &[&i32], which: Half) -> HashSet<i32>{ 
	if sl.is_empty() {
        return HashSet::new();
    }
    let mid = sl.len() / 2;
    match which {
        Half::First => {
            sl[..mid].into_iter().copied().copied().collect()
        }
        Half::Second => {
            sl[mid..].into_iter().copied().copied().collect()
        }
    }
}

// task 7..end..

// task 8 Exercise: Compilation 3
pub fn product_is_even(sl: &[i32]) -> bool {
    sl.into_iter().product::<i32>() % 2 == 0

}
// task 8..end..

// task 9 Exercise: Greater or equal to the sum of all before
pub fn ge_sum_before(sl: &[u32]) -> Vec<bool> {
    sl.into_iter().enumerate()
    .map(|(i,e)| {
    let previous_sum: u32 = sl[..i].iter().sum();
            *e >= previous_sum
        })
        .collect()
}

// task 9..end..

// task 10 Exercise: Compilation 4
pub fn sum_of_prev_plus_curr(sl: &mut [i32]) {
    for i in 0..sl.len() {
        sl[i] += sl[..i].into_iter().sum::<i32>();
    }
}
// task 10..end..