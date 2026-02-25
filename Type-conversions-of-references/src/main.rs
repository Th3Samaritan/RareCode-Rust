use std::collections::HashSet;
fn main() {
    // task 1 clone and ref in one line
    let v: &Vec<i32> = &vec![1, 2, 3];
    let _cv: &Vec<&i32> = &v.into_iter().collect(); // edit this line
    // task 1..end..

    // task 2 Order of operation for &
    	let v: &Vec<i32> = &vec![1,2,3];
	let _cv: &Vec<&i32> = &v.into_iter().collect();
    // task 2..end..

    // task 3 Cloning a reference
    let v: &Vec<i32> = &vec![1, 2, 3];
    let _cv: &Vec<&i32> = &v.iter().collect();
    // task 3..end..

    // task 4 Convert &Vec<i32> into &Set<&i32>
     let v: &Vec<i32> = &vec![1,2,3];

    let _s: &HashSet<&i32> = &v.clone().iter().collect();
    // task 4..end..

    // task 5 Convert &Vec<i32> into &Set<i32> version 1
       let v: &Vec<i32> = &vec![1,2,3];

    let _s: &HashSet<i32> = &v.clone().into_iter().collect();
    // task 5..end..

    // task 6 Convert &Vec<i32> into &Set<i32> version 2
     let v: &Vec<i32> = &vec![1,2,3];

    let _s: &HashSet<i32> = &v.iter().copied().collect();
    // task 6..end..

    // task 7 Convert &Vec<&i32> into Set<i32> Version 1
     let v: &Vec<&i32> = &vec![&1,&2,&3];

    let _s: HashSet<i32> = v.clone().into_iter().copied().collect();
    // task 7..end..

    // task 8 Convert &Vec<&i32> into Set<i32> Version 2
     let v: &Vec<&i32> = &vec![&1,&2,&3];

    let _s: HashSet<i32> = v.into_iter().copied().copied().collect();
    // task 8..end..

    // task 9 Convert &Vec<&i32> into &Set<i32>
    let v: &Vec<&i32> = &vec![&1, &2, &3];

    let _s: &HashSet<i32> = &v.into_iter().copied().copied().collect();
    // task 9..end..

    // task 10 Convert &Vec<i32> into &Vec<&i32>
     // &Vec<i32> to &Vec<&i32>
  let v2: &Vec<i32> = &vec![1,2,3];
  let _vc2: &Vec<&i32> = &v2.iter().clone().collect();
  // task 10..end..
}
