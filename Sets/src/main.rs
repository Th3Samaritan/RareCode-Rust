use std::collections::HashSet;

fn main() {
    // task 1 Introduction to HashSet
    let mut set = HashSet::new();
    set.insert(10);
    println!("{:?}", set);
    // task 1..end..

    // task 2 Checking HashSet Membership
    let mut set = HashSet::new();
    set.insert(10);
    set.insert(8);
    
    println!("{}", set.contains(&10));
    println!("{}", set.contains(&11));
    // task 2..end..

    // task 3 Alternative HashSet Membership Checking
     let mut set = HashSet::new();
    set.insert(10);
    set.insert(8);
    
    println!("{}", (&set).contains(&10));
    println!("{}", (&set).contains(&11));
    // task 3..end..

    // task 4
    let mut set = HashSet::new();
	set.insert(3);
	set.insert(5);
	let n = 6;
	
	let result = has_zero_to_n(&set, n);
	println!("Set {:?} has 0..{}? {}", set, n, result);

	let n2 = 3; // Set does not contain 0, 1, or 2
	let result2 = has_zero_to_n(&set, n2);
	println!("Set {:?} has 0..{}? {}", set, n2, result2);
    // task 4..end..

    // task 5
   let mut set = HashSet::new();
	set.insert(5);
	set.insert(7);
	let v1 = vec![3,5,9]; // Contains 5
	let v2 = vec![1,2,3]; // No common elements
	
	let result1 = exists_in_set(&set, &v1);
	println!("Set {:?} has element from {:?}? {}", set, v1, result1);
	println!("{:?}", v1);
	println!("{:?}", set);

	let result2 = exists_in_set(&set, &v2);
	println!("Set {:?} has element from {:?}? {}", set, v2, result2);
    // task 5..end..

    // task 6 Removing Elements from HashSet
     let mut set = HashSet::new();
    set.insert(10);
    set.insert(8);
    println!("Before remove: {:?}", set);
    set.remove(&10);
    set.remove(&9); 
    set.remove(&1337);

    println!("After remove: {:?}", set);
    
    println!("Contains 10? {}", (&set).contains(&10));
    println!("Contains 8? {}", (&set).contains(&8));
    // task 6..end..

    // task 7 Cloning HashSets
    let mut set = HashSet::new();
    set.insert(3);
    let mut set2 =set.clone();
    set2.insert(4);
    println!("{:?}", set);
    println!("{:?}", set2);
    // task 7..end..

    // task 8
     let v1 = vec![10i64, -20i64, 500i64, i32::MAX as i64, i32::MIN as i64];
    println!("Original i64: {:?}", v1);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v1));

    let v2 = vec![i32::MAX as i64 + 1, i32::MIN as i64 - 1];
    println!("Original i64: {:?}", v2);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v2));

    let v3: Vec<i64> = vec![];
    println!("Original i64: {:?}", v3);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v3));
    // task 8..end..

    // task 9 Cloning HashSets Through References
      let mut set = HashSet::new();
    set.insert(3);
    let set_ref = &set; 
    let mut set2 = set_ref.clone(); 
    set2.insert(4);
    println!("{:?}", set);
    println!("{:?}", set2);
    // task 9..end..

    // task 10
      let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);

    let v_to_remove = vec![2, 4, 6]; 

    println!("Original set: {:?}", set);
    println!("Vector to remove: {:?}", v_to_remove);
    
    let result_set = remove_vector_elements(&set, &v_to_remove);
    
    println!("Result set: {:?}", result_set);
    println!("Original set unchanged: {:?}", set); 
    // task 10..end..

    // task 11
     let v1 = vec![1,2,3,1,4];
    let idx1 = return_index_of_first_duplicate(&v1);
    println!("Vector {:?} -> First dup index: {}", v1, idx1);

    let v2 = vec![1, 2, 5, 5, 3];
    let idx2 = return_index_of_first_duplicate(&v2);
    println!("Vector {:?} -> First dup index: {}", v2, idx2);

    let v3 = vec![1, 2, 3, 4, 5];
    let idx3 = return_index_of_first_duplicate(&v3);
    println!("Vector {:?} -> First dup index: {}", v3, idx3);

    let v4: Vec<i32> = vec![];
    let idx4 = return_index_of_first_duplicate(&v4);
    println!("Vector {:?} -> First dup index: {}", v4, idx4);
    // task 11..end..

    // task 12
    let v = vec![1, 2, 2, 3, 1, 4, 5, 4];
    println!("Original vector: {:?}", v);
    let set = vector_to_set(v);
    println!("Resulting set: {:?}", set);
    // task 12..end..

    // task 13
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec![3, 7, 8];

    println!("{:?} and {:?} disjoint? {}", v1, v2, is_disjoint(&v1, &v2));
    println!("{:?} and {:?} disjoint? {}", v1, v3, is_disjoint(&v1, &v3));
    println!("{:?} and {:?} disjoint? {}", v2, v3, is_disjoint(&v2, &v3));
    println!("{:?} and {:?} disjoint? {}", v1, v1, is_disjoint(&v1, &v1));
    // task 13..end..
} 

// task 4 Check for Range Membership in HashSet
pub fn has_zero_to_n(set: &HashSet<i32>, n: i32) -> bool {
	for i in 0..n{
        if set.contains(&i){
        return true;
        }
    }
    false
} 
// task 4..end..

// task 5 Check if Any Vector Element Exists in Set
pub fn exists_in_set(set: &HashSet<i32>, v: &Vec<i32>) -> bool {
	for i in 0..v.len(){
        if set.contains(&v[i]){
            return true;
        }
    }
    false
} 
// task 5..end..

// task 8 Safe Casting Vector to HashSet
pub fn safe_cast_vec_to_set(v: &Vec<i64>) -> HashSet<i32> {
    let mut set = HashSet::new();
    for i in 0..v.len(){
      if v[i] >= i32::MIN as i64 && v[i]<= i32::MAX as i64{
       set.insert(v[i] as i32);
      }
    }
    set
} 
// task 8..end..

// task 10 Remove Vector Elements from HashSet
pub fn remove_vector_elements(set: &HashSet<i32>, v: &Vec<i32>) -> HashSet<i32> {
    let mut set2 = set.clone();
    for i in 0..v.len(){
        if set2.contains(&v[i]){
            set2.remove(&v[i]);
        }
    }
    set2
} 
// task 10..end..

// task 11 Find First Duplicate in a Vector
pub fn return_index_of_first_duplicate(v: &Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    for i in 0..v.len(){
        if set.contains(&v[i]){
           return i as i32;
        }
       set.insert(v[i]);
    }
    -1
} 
// task 11..end..

// task 12 Convert Vector to HashSet
pub fn vector_to_set(v: Vec<i32>) -> HashSet<i32> {
    let mut s = HashSet::new();
    for i in 0..v.len(){
        s.insert(v[i]);
    }
    s
    }
    // task 12..end..

    // task 13 Check if Two Vectors are Disjoint
pub fn is_disjoint(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    // your code here
    let mut s = HashSet::new();
    for i in 0..v1.len(){
        s.insert(v1[i]);
    }
    for i in 0..v2.len(){ 
        if s.contains(&v2[i]){
        return false;
    }
 }
    true
} 
// task 13..end..