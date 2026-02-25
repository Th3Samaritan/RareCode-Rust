use std::collections::HashMap;

// task 1 HashMaps as arguments and return values
pub fn update_values(mut hm: HashMap<i32, i32>) -> HashMap<i32, i32> {
    hm.insert (2, 6);
    hm.insert (3, 30);
    hm
}
// task 1..end..

// task 4

pub fn without_false(hm: HashMap<i32, bool>) -> HashMap<i32, bool> {

    let mut foo = HashMap::new();

    for (key, value) in hm.into_iter() {
        if value {
            foo.insert(key, value);
        }
    }
    foo
}
// task 4..end..

// task 5
pub fn list_keys(hm: HashMap<i32, i32>) -> Vec<i32> {
    hm.keys().copied().collect()
}
// task 5..end..

// task 6
pub fn sum_keys(hm: HashMap<i32, i32>) -> i32 {
   hm.keys().copied().sum() 
}
// task 6..end..

// task 7 Exercise: create new HashMap with even keys
pub fn even_keys_only(hm: HashMap<i32, i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for key in hm.keys(){
        if key % 2==0 {
            result.push(*key);
        }
    } result
}
// task 7..end..

// task 8
pub fn count_negative_values(hm: HashMap<i32, i32>)->usize{
    let mut count = 0; 
    for val in hm.values(){
        if *val < 0 {
            count +=1; 
        }
    }count
}
// task 8..end..

// task 9 Exercise: Max of Positive Values
pub fn max_value(hm: HashMap<i32, i64>)-> i64 {
   let res = hm.values().max();

   if res.is_none() {
    return 0;
   }
    let val = *res.unwrap();
    if val < 0 {
        0
    } else {
        val
    }


}
// task 9..end..

// task 10
pub fn split_keys_values(hm: HashMap<i32, i32>) ->(Vec<i32>, Vec<i32>) {
    let mut makuli = Vec::new();
    let mut valium = Vec::new();

    for key in hm.keys(){
        makuli.push(*key);
    }
    
    for val in hm.values(){
        valium.push(*val);
        
    }(makuli, valium)
}

// task 10..end..

// task 11 Count Occurrences in Values
pub fn count_items(hm:HashMap<i32, i64>)->HashMap<i64, i64>{
   let mut result = HashMap::new();

   for value in hm.values (){
    let count = result.get(value);
    
    if count.is_some() {
        result.insert(*value, count.unwrap()+1);
    } else {
        result.insert(*value, 1);
    }
   }
   result
} 
// task 11..end..

fn main() {
    // task 1 HashMaps as arguments and return values
    let mut original = HashMap::new();
    original.insert(2, 2);
    original.insert(5, 3);
    original.insert(6, 4);
    original.insert(3, 10);

    let updated = update_values(original);
    println!("{:?}", updated); // {2: 6, 5: 3, 3:30, 6: 4}
    // task 1..end..

    // task 2 Iterating over a HashMap
    let hm = HashMap::from([(1,2),(2,3),(3,4)]);
    
    for (key, value) in hm.iter() {
        println!("{} {}", key, value);
    }
    
    println!("{:?}", hm); // hashmap is consumed;
    // task 2..end..

    // task 3
    let hm = HashMap::from([(1,2),(2,3),(3,4)]);
    
    for (key, value) in hm.iter() {
        foo(key, value);
    }
    
    println!("{:?}", hm); // hashmap is not consumed;
    // task 3..end..

    // task 4 Exercise: create new HashMap without false values
    let v = vec![(1,true), (2, true), (3,false), (4, true)];
    let hm: HashMap<i32, bool> = v.into_iter().collect();
    
    println!("{:?}", without_false(hm));
    // task 4..end..

    // task 5 HashMap .keys()
     let mut data = HashMap::new();
    data.insert(1, 10);
    data.insert(2, 20);
    data.insert(3, 30);

    let keys = list_keys(data);
    println!("{:?}", keys); // [1, 2, 3]
    // task 5..end..

    // task 6 Exercise: Sum keys
    let mut scores = HashMap::new();
    scores.insert(10, 50);
    scores.insert(20, 60);
    scores.insert(30, 70);

    let sum = sum_keys(scores);
    println!("{}", sum); // 60
    // task 6..end..

    // task 7 
    let mut data = HashMap::new();
    data.insert(1, 100);
    data.insert(2, 200);
    data.insert(3, 300);
    data.insert(4, 400);

    let filtered = even_keys_only(data);
    println!("{:?}", filtered); // [2, 4]
    // task 7..end..

    // task 8 HashMap .values()
    let mut data = HashMap::new();
    data.insert(1, -10);
    data.insert(2, 20);
    data.insert(3, -5);
    data.insert(4, 15);

    let result = count_negative_values(data);
    println!("{}", result);
    // task 8..end..

    // task 9 Exercise: Max of Positive Values
    let mut data = HashMap::new();
    data.insert(1, 50);
    data.insert(2, 70);
    data.insert(3, 60);

    println!("{}", max_value(data)); // 70
    // task 9..end..

    // task 10 Exercise: split keys and values
    let mut data = HashMap::new();
    data.insert(1, 10);
    data.insert(2, 20);
    data.insert(3, 30);

    let (keys, values) = split_keys_values(data);
    println!("Keys: {:?}", keys);     // [1, 2, 3]
    println!("Values: {:?}", values); // [10, 20, 30]
    // task 10..end..

    // task 11 Count Occurrences in Values
      let mut data = HashMap::new();
    data.insert(1, 10);
    data.insert(2, 20);
    data.insert(3, 10);
    data.insert(4, 20);
    data.insert(5, 10);

    let counts = count_items(data);
    println!("{:?}", counts); // {10: 3, 20: 2}
    // task 11..end..
}

// task 3 Iterating over references to key and value
pub fn foo(_k: &i32, _v: &i32) {
   
}
// task 3..end..