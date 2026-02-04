// task 1 Introduction to mutable iterators
fn main() {
	let mut v = vec![1,2,3];
	
	for e in v.iter_mut(){
        *e = *e + 1;
	}
	
	println!("{:?}", v);

    // task 1..end..
// task 2 iter_mut() requires a mutable variable
       let mut v = vec![1,2,3];
    for e in v.iter_mut() {
        *e = *e * 6;
    }
    
    println!("{:?}", v);

// task 2..end..

// task 3 Exercise: Double all
let mut v = vec![1,2,3];
    for i in v.iter_mut(){
        *i = *i*2;
    }
	
	println!("{:?}", v);
    // task 3..end..

    /** task 5 Since iter_mut() creates a mutable reference, 
    an immutable reference cannot coexist **/
     
    for e in 0..v.len() {

        for e2 in v.iter() {
            println!("{}", e2);
        }

        v[e] = v[e] * 6;
    }
    
    println!("{:?}", v);
    // task 5..end..

    // task 6 Exercise: Compilation
    
	for e in v.iter_mut() {
	    *e = *e + 1;
	}
	let r = &v;
	println!("{:?}", r);
    // task 6..end..

    // task 7 v.iter_mut() is equivalent to (&mut v).into_iter()
    for e in (&mut v).into_iter() {
		*e = *e * 6;
	}

	println!("{:?}", v);
    // task 7..end..
}

// task 4 Inc then Double

pub fn inc_then_double(mut v: Vec<i32>) -> Vec<i32> {
    
    for e in v.iter_mut() {
        *e = *e + 1;
    }
    for i in v.iter_mut(){
        *i = *i * 2;
    }
   v
}
// task 4..end..

// task 8 Looping through a mutable reference implicitly creates iter_mut()

pub fn double(w: &mut Vec<i32>) {
    
    for e in w {
        *e = *e * 2;
    }
}
// task 8..end..

// task 9 Exercise: Reverse the tuples
fn main() {
	let mut v = vec![(true, false), (false, true), (false, false)];
	
	reverse_all(&mut v);
	println!("{:?}", v); // [(false, true), (true, false), (false, false)]
}

pub fn reverse_all(v: &mut Vec<(bool, bool)>) {
    for i in v {
        let prev1 = i.0;
        let prev2 = i.1;
        *i = (prev2, prev1);
    }
}
// task 9..end..

// task 10 Append Zero To All
ub fn append_0_to_all(v: &mut Vec<Vec<i32>>) {
    
    for i in v{
        i.push(0);
    }
}
// task 10..end..