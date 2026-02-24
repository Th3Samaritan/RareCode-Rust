fn main() {
    // task 1into_iter() consumes the vector
    let v = vec![1, 2, 3];

    let minimum = v.clone().into_iter().min().unwrap();
    let maximum = v.clone().into_iter().max().unwrap();

    println!("{} {}", minimum, maximum);
    // task 1..end..

    //task 2 Iterator on a reference
     let v = vec![1, 2, 3];
    let ref_v = &v;

    let minimum = ref_v.into_iter().min().unwrap();
    let maximum = ref_v.into_iter().max().unwrap();

    println!("{} {}", minimum, maximum);
    // task 2..end..

    // task 3 For Loop on a Reference
     let v = vec![1, 2, 3];

    // version 1 -- implicit into_iter()
    let mut total1 = 0;
    for e in &v {
        total1 += *e;
    }

    // version 2 -- explicit into_iter()
    let mut total2 = 0;
    for e in (&v).into_iter() {
        total2 += *e;
    }

    // version 3 -- into_iter() with iterator method

    let total3: i32 = (&v).into_iter().sum();

    println!("{} {} {}", total1, total2, total3);
    // task 3..end..

    // task 4
    let v = vec![1, 2, 3];

    let result = sum_and_product(v);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Iterating on a reference returns references
     let v = vec![1, 2, 3];
    for e in &v {
        do_nothing(*e);
    }
    // task 5..end..

    // task 6 Iterating on a reference returns references (explicit conversion)
     let v = vec![1, 2, 3];
    for e in (&v).into_iter() {
        do_nothing(e);
    }
    // task 6..end..

    // task 7 Easier Syntax: iter()
    let v = vec![1, 2, 3];

    for e in v.iter() {
        println!("{}", e);
    }

    println!("v is not consumed {:?}", v);
    // task 7..end..

    // task 8 Find Maximum Value in a Vector
    let v = vec![1,2,3];
	
	let result = get_max(&v);
	println!("{:?}", v);
	println!("{}", result);
    // task 8..end..

    // task 9
     let v = vec![1, 2, 3];
    for e in v.iter() {
        do_nothing(e);
    }
    // task 9..end..

    // task 10
     let v = vec![1, 2, 3];
    for e in v.iter() {
        do_nothing(*e); // edit this line
    }
    // task 10..end..

    // task 11
     let v = vec![1, 2, 3];

    let result = get_max(&v);

    println!("{:?}", result);

    // task 11..end..

    // task 12
     let v = vec![1, 2, 3];

    let result = get_second(&v);

    println!("{:?}", result);
    // task 12..end..

    // task 13
    let v = vec![1, 2, 3];

    let result = get_last(&v);

    println!("{:?}", result);
    // task 13..end..

    // task 14
     let v = vec![1, 2, 3];
    let ref_v = &v;

    for e in ref_v.iter() {
        do_nothing(e);
    }
    // task 14..end..

    // task 15
    let v = vec![1, 2, 3];
    let ref_v = &v;

    for e in ref_v.iter() {
        do_nothing(e);
    }
    // task 15..end..

    // task 16
      let v = vec![1, 2, 3];

    for e in v.into_iter() {
        foo(e);
    }
    // task 16..end..

    // task 17
     let v = vec![1, 2, 3];

    for e in v.iter() {
        foo(e);
    }
    // task 17..end..

    // task 18
     let hm = HashMap::from([(1, 10), (2, 20), (3, 30)]);

    let v = vec![2, 3];

    let result = associated_values(hm, v);
    println!("{:?}", result);
    // task 18..end..
}

// task 4 Exercise: Sum and Product of a Vector
pub fn sum_and_product(v: Vec<i32>) -> (i32, i32) {
    // your code here
    let sum = (&v).into_iter().sum();
    let product = (&v).into_iter().product();
    (sum, product)
}
// task 4..end..

// task 5 Iterating on a reference returns references
pub fn do_nothing(_v: i32) {
    
}
// task 5..end..

// task 6
pub fn do_nothing(_v: &i32) {

}
// task 6..end..

// task 8

pub fn get_max(v: &Vec<i32>) -> i32 {

	let mut max = v[0];
     for i in 0..v.len(){
        if v[i] > max {
            max = v[i];
        }
     } 
	// TODO
	max
} 
// task 8..end..

// task 9 Exercise: Function Signature with iter()
pub fn do_nothing(_v: &i32) {

}
// task 9..end..

// task 10 Exercise: Fix the code

// <do not edit>
pub fn do_nothing(_e: i32) {}
// </do not edit>

// task 10..end..

// task 11

pub fn get_max(v: &Vec<i32>) -> Option<i32> {
    // your code here
    if v.is_empty(){
        return None;
    }
    let  v1 = v.iter().max().unwrap();
    Some(*v1)
}
// task 11..end..

// task 12 Exercise: Fix Compilation
pub fn get_second(v: &Vec<i32>) -> Option<i32> {
    if v.len() < 2 {
        return None;
    }
    let scnd = v.iter().nth(1);
    scnd.copied()
}

// task 12..end..

// task 13 Exercise: Fix the code of get last
pub fn get_last(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        return None;
    }
    let last = v.iter().last().unwrap();
    let last1 = *last;
    Some(last1)
}
// task 13..end..

// task 14 Calling .iter() on a reference behaves the same as .into_iter()
pub fn do_nothing(_e: &i32) {

}
// task 14..end..

// task 15 Exercise: .iter() on a reference
pub fn do_nothing(_e: &i32) {

}
// task 15..end..

// task 16 Exercise: iter() or into_iter() 1
pub fn foo(_e: i32) {

}
// task 16..end..

// task 17
pub fn foo(_e: &i32) {

}
// task 17..end..

// task 18 Exercise: iter() or into_iter() 3
pub fn associated_values(hm: HashMap<i32, i32>, v: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    for e in v.iter() {
        if hm.get(e).is_some() {
            ret.push(*hm.get(e).unwrap());
        }
    }
    ret
}

// task 18..end..