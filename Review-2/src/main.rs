// task 1 Review: Increment all keys of the HashMap
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::from([(1, 10), (2, 20)]);

    inc_all_keys(&mut map);
    
    println!("{:?}", map);
}

pub fn inc_all_keys(m: &mut HashMap<i32, i32>) {
    let mut n = HashMap::new();
    for (k,v) in m.iter_mut(){
        n.insert(k + 1, *v);
    }
    *m = n;
}

// task 2 Compilation: Remove the Max Value from the Set

fn main() {
	let mut s = HashSet::from([1,2,3]);
	
	remove_max(&mut s);
	println!("{:?}", s);
}

pub fn remove_max(s: &mut HashSet<i32>) {
    let max: Option<i32> = s.iter().max().copied();

    if max.is_some() {
        s.remove(&max.unwrap());
    }
}
// task 2..end..

// task 3 Review: Increment HashMap A by B

pub fn increment_by_hashmap(a: &mut HashMap<i32, i32>, b: HashMap<i32, i32>) {
    for val in a.values_mut(){
        if b.contains_key(val){
           let c = b.get(val).unwrap();
            *val=*val + c;
        }
    } 
}

// task 3..end..

// task 4 Review: Double Ref Vec to HashMap
 
fn main() {
    let v = &&vec![&1,&2,&3];
    let w = &&vec![10,20,30];
    
    let result = create_map(v, w);
    println!("{:?}", result);
}

pub fn create_map(v: &&Vec<&i32>, w: &&Vec<i32>) -> HashMap<i32, i32> {
    let mut m = HashMap::new();
   for (i, k) in (*v).into_iter().enumerate(){
    m.insert(**k, w[i]);
   }
   m
}
// task 4..end..

// task 5 Review: Fix Compilation

fn main() {
    let m = &HashMap::from([(1,2),(3,4),(5,1)]);
    
    for (k, v) in m {
        accept(k, v);
    }
}

pub fn accept(_k:&i32, _v:&i32) {}

// task 5..end..

// task 6 Review: Values as Vec

fn main() {
    let m = HashMap::from([(1,2),(3,4),(5,1)]);
    let result = values_as_vec(&m);
    println!("{:?}", result);
}

pub fn values_as_vec(m: &HashMap<i32, i32>) -> Vec<i32> {
    m.values().copied().collect()
}
// task 6..end..


//task 7 Review: Return the product of all values in the hashmap

pub fn prod_values(m: &HashMap<i32, i32>) -> i32 {
   m.values().product()
}
// task 7..end..