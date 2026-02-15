use std::collections::HashMap;
// task 5
pub enum InventoryStatus {
    Available(u64),
    Preorder,
    OutOfStock,
}

// task 5..end..

// task 6 if let with enum variant that doesn’t wrap a value
pub enum Color {
    Black,
    White,
}
// task 6..end..
fn main() {
    // task 1 if let syntax
    let v = vec![Some(1), Some(2), None, Some(3)];
    
    let result = sum_somes(v);
    println!("{}", result);
    // task 1..end..

    // task 2 if let Some with a reference
     let op = Some(20);
    let result = twice_or_zero(&op);
    println!("{}", result);
    // task 2..end..

    // task 3 Exercise: if let Some with a mutable reference
    let mut m = HashMap::from([(1, 100), (2, 200), (3, 300)]);
    
    let k = 3;
    
    inc_if_present(&mut m, &k);
    
    println!("{:?}", m);
    // task 3..end..

    // task 4 if let with Result
    let n = "49151";
    
    let result = is_valid_tcp(n);
    println!("{}", result);
    // task 4..end..

    // task 5
     let item_status = InventoryStatus::Available(100);
    
    let result = quantity_available(&item_status);
    println!("{}", result);
    // task 5..end..

    // task 6 if let with enum variant that doesn’t wrap a value
    let c = Color::Black;
    let result = is_black(c);
    println!("{}", result);
    // task 6..end..

    // task 7 Increment At
    let mut v = vec![1, 2, 3];
    
    inc_at(&mut v, 2);
    println!("{:?}", v);
    // task 7..end..
}

// task 1 if let syntax
pub fn sum_somes(v: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    
    for opt in v {
        if let Some(n) = opt {
            sum += n;
        }
    }
    sum
}
// task 1..end..

// task 2 if let Some with a reference

pub fn twice_or_zero(op: &Option<i32>) -> i64 {
    if let Some(v) = *op {
        return i64::from(v) * 2;
    }
    0
}
// task 2..end..

// task 3 Exercise: if let Some with a mutable reference
pub fn inc_if_present(m: &mut HashMap<i32, i32>, k: &i32) {
    if let Some(v) = m.get_mut(k){
        *v +=1;
    }
}
// task 3..end..

// task 4 if let with Result
pub fn is_valid_tcp(n: &str) -> bool {

    if let Ok(port) = n.parse::<u16>(){
        if port >= 1024 && port <= 49151 {
            return true;
        }
    }
    false
}
// task 4..end..

// task 5 if let with custom enum
pub fn quantity_available(item: &InventoryStatus) -> u64 {
    if let InventoryStatus::Available(x)=item{
       return *x;
    } else {
        0
    }
}
// task 5..end..

// task 6 if let with enum variant that doesn’t wrap a value
pub fn is_black(c: Color) -> bool {
    if let Color::Black = c {
        return true;
    }
    false
}
// task 6..end..

// task 7 Increment At
pub fn inc_at(v: &mut Vec<i32>, index:usize) {

    if let Some(x) = v.get_mut(index){
       *x += 1;
    }
}
// task 7..end..