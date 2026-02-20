
use std::fmt::Debug;
use std::collections::{HashSet, HashMap};
use std::hash::Hash;
// task 1 Multiple Constraints
pub fn all_or_empty<T: Clone + Debug>(v: Vec<T>) -> Vec<T> {
    if v.len() > 3 {
        return v.clone();
    }
    
    println!("{:?} is too short", v);
    Vec::new()
}
// task 1..end..

// task 2
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum E {
    A,
    B,
    C,
}

pub fn to_set<T: Clone + Eq + Hash>(v: Vec<T>) -> HashSet<T> {
    v.into_iter().collect()
}
// task 2..end..

//  task 3 References to Owned Set
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum E {
    A,
    B,
    C,
}

pub fn to_set<T: Copy + Eq + Hash>(v: &[T]) -> HashSet<T> {
    v.into_iter().copied().collect()
}

// task 3..end..

// task 4 Exercise: Generic has_duplicates
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum E {
    A,
    B,
    C,
}

pub fn has_duplicates<T: Hash + Eq+ Copy>(v: &[T]) -> bool{
    let mut set = HashSet::new();
    for e in v {
    if set.contains(e) {
        return true; 
    }
    set.insert(*e);
}false
}
// task 4..end..

// task 5 Hashmap

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum E {
    A,
    B,
    C,
}

pub fn create_hash<T: Eq + Hash, U:Copy >(k: Vec<T>, v: Vec<U>) -> HashMap<T, U> {
    let mut map = HashMap::new();
    for (i, e) in k.into_iter().enumerate(){
        map.insert(e, v[i]);
    }
    map
}
// task 5..end..

// task 6
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum E {
    A,
    B,
    C,
}

pub fn reverse<T:Hash+ Eq, U: Hash+Eq>(m: HashMap<T, U>) -> HashMap<U, T>{
    let mut hm = HashMap::new();
    for (k, v) in m {
        hm.insert(v, k);
    }
    hm
}
// task 6..end..

// task 7
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Account {
    pub balance: u32,
}

pub fn anagram<T: Eq + Hash + Clone >(sl1: &[T], sl2: &[T])->bool{
     let mut m1 = HashMap::<T, usize>::new();
    let mut m2 = HashMap::<T, usize>::new();
    for e in sl1 {
        let count = m1.get(e).unwrap_or(&0);
        m1.insert(e.clone(), count + 1);
    }
    for e in sl2 {
        let count = m2.get(e).unwrap_or(&0);
        m2.insert(e.clone(), count + 1);
    }
    m1 == m2
}
// task 7..end..

fn main() {
    // task 1
    let v = vec![1,2,3,4];
    let result = all_or_empty(v);
    
    println!("{:?}", result);
    
    let v = vec!["a".to_string(), "b".to_string()];
    let result = all_or_empty(v);
    
    println!("{:?}", result);
    // task 1..end..

    // task 2 Hash Trait
    let v = vec![E::A, E::C, E::C, E::B];
    let result = to_set(v);
    println!("{:?}", result);
    // task 2..end..

    // task 3 References to Owned Set
        let v = vec![E::A, E::C, E::C, E::B];
    let result = to_set(&v);
    println!("{:?}", result);
    // task 3..end..

    // task 4 Exercise: Generic has_duplicates
     let v = vec![1, 3, 3, 2, 2];
    let result = has_duplicates(&v);
    println!("{:?}", result);
    
    let v = vec![E::A, E::A];
    let result = has_duplicates(&v);
    println!("{:?}", result);
    // task 4..end..

    // task 5 Hashmap
     let k = vec![E::A, E::C, E::C, E::B];
    let v = vec![1, 3, 3, 2];
    let result = create_hash(k, v);
    println!("{:?}", result);
    
    let k = vec![E::A, E::C, E::C, E::B];
    let v = vec![1.0, 3.0, 3.0, 2.0];
    let result = create_hash(k, v);
    println!("{:?}", result);
    // task 5..end..

    // task 6 Reverse HashMap
    let v = HashMap::from([(E::A, 1), (E::C, 2), (E::C, 2)]);
    let result = reverse(v);
    println!("{:?}", result);
    // task 6..end..

    // task 7 Generalized Anagram
    fn main() {
    let l1 = ["hello".to_string(), "bye".to_string(), "bye".to_string()];
    let l2 = ["bye".to_string(), "hello".to_string(), "bye".to_string()];
    
    let result = anagram(&l1, &l2);
    println!("{}", result);
    
    let l1 = [Account { balance: 3 }, Account { balance: 3}, Account { balance: 4 }];
    let l2 = [Account { balance: 4 }, Account { balance: 3}, Account { balance: 3 }];
    
    let result = anagram(&l1, &l2);
    println!("{}", result);
}
// task 7..end..
}