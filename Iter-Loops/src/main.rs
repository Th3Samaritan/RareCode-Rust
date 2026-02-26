use std::collections::HashMap;
fn main() {
    // task 1 For loop without consumption
  let v = vec![1,2,3];
  
  for e in v.iter() {
    println!("{}", e);
  }
  
  println!("v is not consumed {:?}", v);
  // task 1..end..

  // task 2 for e in &v vs for e in v.iter()
  let v = vec![1,2,3];
  
  for e in (&v).into_iter() {
    accept(e);
  }
  
  println!("v is not consumed {:?}", v);
  // task 2..end..

  // task 3 Exercise: Idiomatic Iteration
  let v = vec![1,2,3];
	
	for e in &v {
	    accept(e);
	}
    // task 3..end..

    // task 4 Iter with skips
    let v = vec![1,2,3,4];
		
	for e in v.iter().step_by(2) {
	    println!("{}", e);   
	}
	
	println!("{:?}", v);
    // task 4..end..

    // task 5 Iter reversed
    let v = vec![1,2,3,4];
		
	for e in v.iter().rev() {
	    println!("{}", e);
	}
	
	println!("not consumed: {:?}", v);
    // task 5..end..

    // task 6 Exercise: HashMap Iter Type
    let hm = HashMap::from([(1,2),(3,4),(5,6)]);
	
	for (k, v) in &hm {
	    accept1(k, v);
	}
	
	for e in &hm {
	    accept2(e);
	}
    // task 6..end..
}

// task 2 for e in &v vs for e in v.iter()

fn accept(_v: &i32) {}
// task 2..end..

// task 3 Exercise: Idiomatic Iteration
fn accept(_e: &i32) {}
//task 3..end..

// task 6 Exercise: HashMap Iter Type
fn accept1(_k: &i32, _v: &i32) {}

fn accept2(_e: (&i32, &i32)) {}
// task 6..end..
