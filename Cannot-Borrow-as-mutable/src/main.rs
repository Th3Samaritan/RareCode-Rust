use std::collections::HashSet;
 task 1 variables cannot be mutated if another variable has a reference to it
fn main() {
    let mut v = vec![1, 2, 3];
    
    let _r = &v;
    v.push(4);
    //println!("{:?}", r);

    // task 2 Reference to mutable variable restriction
    /*Rearranged the code*/
    let mut v = vec![1, 2, 3];
      v.push(4);
    let r = &v;
    println!("{:?}", r);
    // task 2..end..
}
// task 3 Exercise: Mutable Borrow use later

fn main() {
    let mut s = HashSet::from([1,2,3]);
    
    let ref_s = &s;
    let result_1 = sum(ref_s);
    println!("{}", result_1);
    
    s.insert(4);
    
    //println!("{}", sum(ref_s));

    // task 4 References going out of scope
    
	//reference created here
    let result_1 = &s; // your code here
    // reference goes out of scope
    
    println!("{:?}", result_1);
    s.insert(4);
    println!("{:?}", sum(&s));
    //  task 4..end..
}

pub fn sum(s: &HashSet<i32>) -> i32 {
    s.iter().sum()
}
 
//Creating an iterator of references creates an immutable reference

fn main() {
    let mut s: HashMap<i32, i32> = HashMap::from([(1, 10), (2, 20)]);

    s.insert(3, 30);
    let keys_iter = s.keys();
    
    
    let keys_vec: Vec<i32> = keys_iter.copied().collect();
    println!("{:?}", keys_vec);
}

// task 7 Exercise: modification inside an iter loop

fn main() {
    
    let a: HashMap<i32, i32> = HashMap::from([(2, 10), (3, 30)]);
    
    let result = remove_even_keys(a);
    println!("{:?}", result);
}

pub fn remove_even_keys(mut a: HashMap<i32, i32>) -> HashMap<i32, i32> {
    let keys: Vec<i32> = a.keys().copied().collect();
    for k in keys {
        if k % 2 == 0{
            a.remove(&k);
        }
    }
    a
}
// task 7..end..

// task 9 Exercise: inc HashMap values

fn main() {
    
    let mut hm = HashMap::from([(1,2), (2, 4), (3, 6)]);
    
    inc_all_values(&mut hm);
    println!("{:?}", hm);
}

pub fn inc_all_values(hm: &mut HashMap<i32, i32>) {
    
    for (k, v) in hm.iter() {
        hm.insert(*k, v + 1);
    }
}

// task 9..end..

// task 10 Execise: Vector repeated

pub fn self_append(mut v: Vec<i32>) -> Vec<i32> {
   let v_copy = v.clone(); 
    for e in v_copy {
        v.push(e);
    }
    
    v
}

// task 10..end..