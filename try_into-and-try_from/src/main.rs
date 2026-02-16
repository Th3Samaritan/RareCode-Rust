use std::collections::{HashSet, HashMap};

fn main() {
    // task 1 Larger to smaller integer
    let set = HashSet::from([1,2,3,256,4]);
    let result = downcast_all(set);
    println!("{:?}", result);
    // task 1..end..

    // task 2 Set to iterator to array
    let ok = HashSet::from([1, 2, 3]);
    let too_many = HashSet::from([1, 2, 3, 4]);

    println!("{:?}", set_to_array3(&ok));  
    println!("{:?}", set_to_array3(&too_many)); 
    //  task 2..end..

    // task 3 Exercise: Possibly valid string
    let data = vec![0x41, 0x1F600, 0xD800, 0x5A];
    let s = to_lossy_string(data);
    println!("{}", s);
    // task 3..end..

    // task 4 Exercise: Saturating Cast
    let v = vec![10, 70000, 65535, 0];
    let out = saturating_downcast(v);
    println!("{:?}", out);
    // task 4..end..

    // task 5 Exercise: Values of a HashMap cast to an array
     let mut m: HashMap<&str, u64> = HashMap::new();
    m.insert("a", 1);
    m.insert("b", 2);
    m.insert("c", 3);

    println!("{:?}", values_to_array3(&m)); 

    m.insert("d", 4);
    println!("{:?}", values_to_array3(&m));
    // task 5..end..
}

// task 1 Larger to smaller integer
pub fn downcast_all(set: HashSet<u16>) -> HashSet<u8> {
 set.into_iter().map(|x| u8::try_from(x)).filter(|res| res.is_ok()).map(|res| res.unwrap()).collect()
}
// task 1..end..

// task 2 Set to iterator to array
pub fn set_to_array3(set: &HashSet<i32>) -> Option<[i32; 3]> {
    let v: Vec<i32> = set.iter().cloned().collect();
    match v.try_into() {
        Ok(arr) => Some(arr),
        Err(_) => None,
    }
}
// task 2..end..

// task 3 Exercise: Possibly valid string
pub fn to_lossy_string(v: Vec<u32>) -> String {
    v.into_iter().map(|n| {
    match char::try_from(n) {
        Ok(c) => c,
        Err(_) => '�',
    }
}).collect()
}
// task 3..end..

// task 4 Exercise: Saturating Cast
pub fn saturating_downcast(v: Vec<u32>) -> Vec<u16> {
     v.into_iter()
        .map(|n| match u16::try_from(n) {
            Ok(val) => val,
            Err(_) => u16::MAX,
        })
        .collect()
}
// task 4..end..

// task 5 Exercise: Values of a HashMap cast to an array
pub fn values_to_array3(map: &HashMap<&str, u64>) -> Option<[u32; 3]> {
    if map.len() != 3 {
        return None;
    }

    let mut vals = Vec::new();
    for &v in map.values() {
        if let Ok(n) = u32::try_from(v) {
            vals.push(n);
        } else {
            return None;
        }
    }
    match vals.try_into() {
        Ok(arr) => Some(arr),
        Err(_) => None,
    }
}
// task 5..end..