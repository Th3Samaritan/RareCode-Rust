use std::collections::HashMap;
// task 1 Only one mutable reference can exist at a time
fn main() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v;
     println!("{:?}", r1);
    let r2 = &mut v;
    println!("{:?}", r2);
    // task 1..end..
    // task 2 Mutating a mutable variable while a mutable reference exists
     v[0] = 2;
    let r1 = &mut v;    
    println!("{:?}", r1);
    // task 2..end..

    // task 3 Cannot modify a collection while iterating with mutable references
    for i in 0..v.len() {
        if v[i] > 2 {
            v.remove(i);
        }
    }
    println!("{:?}", v);
    // task 3..end..

    // task 4 A mutable reference is not a copy type
     let r1 = &v;
    let r2 = r1;

    println!("{:?}", r1);
    println!("{:?}", r2);
    // task 4..end..

    // task 5
     let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1337, 42);
    map.insert(9000, 100);
    let vals_mut = map.values_mut();
    // task 5..end..

    // task 6 Exercise: Mutable Reference to a 2D Vector
     let mut grid = vec![vec![1, 2], vec![3, 4]];

    let row1 = &mut grid[0]; // Mutable borrow of first row
     row1[1] = 10;
    let cell = &mut grid[1][0]; // Attempts mutable borrow of a cell in second row, but since grid is borrowed via row1, it's subtle if thinking rows are independent   
    *cell = 20;
    // task 6..end..
}
 // task 5
    for v in vals_mut {
        *v = *v + 1;
    }
// task 5..end..