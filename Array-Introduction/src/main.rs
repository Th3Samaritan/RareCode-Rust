// task 1 Array introduction
fn main() {
    
    let a = [true, false, true, false];
    accept(a);
// task 3 Mutable arrays
     let mut a = [1,2,3];
    a[0] = 10;
    println!("{:?}", a);
    // task 3..end..

    // task 4 Array of non-Copy Types
	let a: [Vec<i32>; 3] = [vec![1, 2], vec![2], vec![3]];
	take(a.clone());
	println!("a not consumed {:?}", a);
    // task 4..end..

    //task 5 Arrays can support iteration
    let mut a: [i32; 3] = [1,2,3];
    
    let result = sum(a);
    println!("{}", result);
    
    inc_all(&mut a); // [2,3,4];
	  println!("{:?}", a);
    // task 5..end..

    // task 6 Array of copy type
     let a: [i32; 3] = [1,2,3];
    
    // none of these take ownership of `a`
    take(a);
    let a1 = a;
    let a2 = a;
    // your code here
    
    println!("{:?}", a1);
    println!("{:?}", a2);
    // task 6..end..

    // task 7 .into_iter() when for Copy type arrays
    let a: [i32; 3] = [1,2,3];
    
    for e in a {
        println!("{}", e);
    }
    
    println!("{:?}", a);
    // task 7..end..

    // task 8 .into_iter() when for non-Copy type arrays
     let a: [Vec<i32>; 3] = [vec![1],vec![2],vec![3]];
     println!("{:?}", a);
    for e in a {
        println!("{:?}", e);
    }
    // task 8..end..

    // task 9 Exercise: iterator methods on an array
    let a = [1,2,5,4,3];
    
    let result = coverage(a);
    println!("{}", result);
        // task 9..end..

    // task 10 Cannot create an array from an iterator
    let v: Vec<i32> = (0..=2).into_iter().collect();
    // let a: [i32; 3] = (0..=2).into_iter().collect();
    println!("{:?}", v)
    // task 10..end..

    // task 11 Exercise: Overwrite
     let mut a = [1,2,5,4,3];
    
    overwrite(&mut a);
    println!("{:?}", a)
    // task 11..end..

    // task 12 Exercise: Overwrite 2
    let mut a = [1, 2, 5, 4, 3];
    let b = [&&4, &&5, &&6, &&7, &&8];
    
    overwrite_v2(&mut a, b);
    println!("{:?}", a);
    // task 12..end..
}

pub fn accept(_t: [ bool; 4]) {}   
// task 1..end..

 // task 4 Array of non-Copy Types
pub fn take(_a: [Vec<i32>; 3]) {}
// task 4..end..

// task 5 Arrays can support iteration
// add one to each element
pub fn inc_all(a: &mut [i32; 3]) {
	for e in a {
        *e += 1;
    }
}

pub fn sum(a: [i32; 3]) -> i32 {
    a.iter().sum()
}
// task 5..end..

// task 6 Array of copy type

fn take(_a: [i32; 3]) {}
// task 6..end..

// task 9 Exercise: iterator methods on an array

pub fn coverage(a: [i32; 5]) -> i32 {
		let max_val = a.into_iter().max().unwrap();
        let min_val = a.into_iter().min().unwrap();
        let cov = max_val-min_val;
        cov
}
// task 9..end..

// task 11 Exercise: Overwrite

pub fn overwrite(a: &mut [i32; 5]) {
    for (i, e) in a.iter_mut().enumerate(){
        *e = i as i32;
    }
}
// task 11..end..

// task 12 Exercise: Overwrite 2

pub fn overwrite_v2(a: &mut [i32; 5], b: [&&i32; 5]) {
    for (i, e) in a.iter_mut().enumerate(){
        *e = **b[i];
    }
}
// task 12..end..

// task 13 Tic Tac Toe
fn main() {
	let board = [[1,2,1],
	             [2,1,2],
	             [1,2,1]];
	             
	let result = is_win(board, 1);
	println!("{}", result);
}

pub fn is_win(b: [[u8; 3]; 3], player: u8) -> bool {
    for i in 0..3 {
        if b[0][i] == player && b[1][i] == player && b[2][i] == player {
            return true;
        }
    }
    for i in 0..3 {
        if b[i][0] == player && b[i][1] == player && b[i][2] == player {
            return true;
        }
    }
    if b[0][0] == player && b[1][1] == player && b[2][2] == player {
        return true;
    }
    if b[0][2] == player && b[1][1] == player && b[2][0] == player {
        return true;
    }

    false
}
// task 13..end..

// task 14 Exercise: Transpose
fn main() {
	let mut matrix = [[1,2,3],[4,5,6],[7,8,9]];
	
	transpose(&mut matrix);
	println!("{:?}", matrix);
}

pub fn transpose(matrix: &mut [[i32; 3]; 3]) {
    for i in 0..3 {
        for j in (i + 1)..3 {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
}
// task 14..end..

// task 15 Exercise: Row to zero
fn main() {
	let mut a = [[1,1,1,1],[2,2,2,0],[3,3,3,3],[0,0,4,4]];
	
	row_to_zero(&mut a);
	println!("{:?}", a);
}

pub fn row_to_zero(a: &mut [[i32; 4]; 4]) {
	 for row in a {
        if row.contains(&0) {
            *row = [0, 0, 0, 0];
        }
    }
}
// task 15..end..