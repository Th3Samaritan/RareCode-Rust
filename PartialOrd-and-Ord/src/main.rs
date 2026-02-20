// introduction to PartialOrd 
pub fn max<T: PartialOrd >(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}
// task 1..end..

// task 2 PartialOrd with Enums
#[derive(Debug, PartialEq, PartialOrd)]
pub enum HeavenlyObject {
    Star,
    Moon,
    Sun
}

pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}
// task 2..end..

// task 3 Custom Order in Enums
#[derive(Debug, PartialEq, PartialOrd)]
pub enum HeavenlyObject {
    Sun = 2,
    Moon = 1,
    Star = 0,
    Galaxy = 3
}

pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

// task 3..end..

// task 4 PartialOrd Requires PartialEq
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ImperialVolume {
    Gallon = 16,
    Quart = 4,
    Pint = 2,
    Cup = 1,
}
// task 4..end..

// task 5 PartialOrd applied to Structs
use std::cmp::PartialOrd; 
#[derive(Debug, PartialOrd, PartialEq)]
pub struct S {
    pub first_field: i32,
    pub second_field: i32,
}

pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}
// task 5..end..

// task 8 Deriving Ord
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct S {
  pub  f: u32,
}
// task 8..end..

// task 9
pub fn is_permutation<T: Clone +Ord>(s1: &[T], s2: &[T]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    
    let mut a = s1.to_vec();
    let mut b = s2.to_vec();
    
    a.sort();
    b.sort();

    a==b
}

// task 9..end..

// task 10
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum VariableNames {
    Foo,
    Bar,
    Baz,
    Qux,
    Quux,
}

 pub fn contains_duplicates<T: Clone + Ord>(sl: &[T]) -> bool {
    if sl.len() < 2 {
        return false;
    }
    let mut a = sl.to_vec();
    a.sort();
    for i in 0..(a.len() - 1) {
        if a[i] == a[i + 1] {
            return true;
        }
    }
    
    false
}
// task 10..end..

fn main() {
    // task 1 
    let result = max('a', 'c');
    println!("{}", result);

    let result = max(true, false);
    println!("{}", result);
    
    let result = max(3.0, 2.0);
    println!("{}", result);
    // task 1..end..

    // task 2 PartialOrd with Enums
     let max_obj = max(HeavenlyObject::Moon, HeavenlyObject::Star);
    let min_obj = min(HeavenlyObject::Moon, HeavenlyObject::Star);
    println!("{:?} is greater than {:?}", max_obj, min_obj);
    
    let max_obj = max(HeavenlyObject::Sun, HeavenlyObject::Moon);
    let min_obj = min(HeavenlyObject::Sun, HeavenlyObject::Moon);
    println!("{:?} is greater than {:?}", max_obj, min_obj);
    // task 2..end..

    // task 3 Custom Order in Enums
     let result = max(HeavenlyObject::Moon, HeavenlyObject::Star);
    println!("{:?}", result);
    // task 3..end..

    // task 4 PartialOrd Requires PartialEq
     let result = ImperialVolume::Quart > ImperialVolume::Cup;
    println!("{:?}", result);
    // task 4..end..

    // task 5 PartialOrd applied to Structs
    let s1 = S {
        first_field: 10,
        second_field: 20,
    };
    let s2 = S {
        first_field: 10,
        second_field: 23,
    };
    let result = max(s1, s2);
    println!("{:?}", result);
    // task 5..end..

    // task 6 Sorting
    let mut a = [4,3,6,1];
    let sl_a = &mut a;
    
    sl_a.sort();
    
    println!("{:?}", a);
    // task 6..end..

    // task 8 Deriving Ord
    let s1 = S { f: 3 };
    let s2 = S { f: 1 };
    let s3 = S { f: 2 };
    
    let mut a = [s1, s2, s3];
    a.sort();
    
    println!("{:?}", a);
    // task 8..end..

    // task 9 Exercise: IsPermutation — Sort Edition
     let a = [1,1,2,3,4];
    let b = [4,3,2,1,1];
    
    let result = is_permutation(&a, &b);
    
    println!("{:?}", result);
    // task 9..end..

    // task 10 Contains Duplicates Sort Edition
     let a = [VariableNames::Quux, VariableNames::Foo, VariableNames::Qux, VariableNames::Baz, VariableNames::Qux, VariableNames::Bar];
    
    let result: bool = contains_duplicates(&a);
    println!("{}", result);
    // task 10..end..
}