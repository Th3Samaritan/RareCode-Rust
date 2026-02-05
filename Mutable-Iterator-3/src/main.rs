// task 2 Mutable iteration on a HashMap
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut hm = HashMap::from([(1,10), (2, 20), (3, 30)]);
    
    for (_k, v) in &mut hm {
        *v = *v + 1;
        // *k = *k + 1;
    }
    
    println!("{:?}", hm);

    // task 2..end..
    
    // task 3 .Values_mut() for HashMap
       for val in map.values_mut() {
        *val = *val + 5;
    }
    
    println!("{:?}", map);
    // task 3..end..
}

// task 4 HashSet mutable iterator workaround

fn main() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let mut v: Vec<i32> = set.into_iter().collect();
    for e in &mut v {
        *e *= 2; // equivalent to *e = *e * 2;
    }
    set = v.into_iter().collect();
    
    println!("{:?}", set);
}
// task 4..end..

// task 5 Exercise: Cleaner HashMap Iteration

fn main() {
    let mut map: HashMap<usize, i32> = HashMap::from([(1,10),(2,20)]);
    
    for  v in  map.values_mut() {
        *v += 100;
    }
    
    println!("{:?}", map);
}
// task 5..end..

// task 6 Iterator consumption review

pub fn add_15(m: &mut HashMap<i32, i32>) {
    for v in m.values_mut() {
        *v += 5;
    }
    
    for  v in m.values_mut() {
        *v += 10;
    }
}
// task 6..end..

// task 7 Exercise: add key to value

pub fn add_the_key(m: &mut HashMap<usize, i32>) {
    
    for (k, v) in m.iter_mut(){
        *v = *v + (*k as i32);
    }
}
// task 7..end..