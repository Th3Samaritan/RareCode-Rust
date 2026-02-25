fn main() {
    // task 1 A clone on a reference
    let v = vec![1, 2, 3];

    // ref_v is of type &Vec<i32>
    let ref_v: &Vec<i32> = &v;
    
    // result is of type Vec<i32>
    let _result: Vec<i32> = ref_v.clone();
    // task 1..end..

    // task 2 A clone on a double reference 1
    let v = vec![1,2,3];
	let ref_ref_v = &&v;
	
	// clone "removes" one & and creates a &Vec<i32> not a Vec<i32>
	let _v: Vec<i32> = (*ref_ref_v).clone();
    //task 2..end..

    // task 3 A clone on a double reference 2
	let v = vec![1,2,3];
	let ref_ref_v = &&v;
	
	let _result: Vec<i32> = (*ref_ref_v).clone();
    // task 3..end..

    // task 4 Copy types and cloning
    let a = 2;
	let ref_a = &a;
	
	let _deref_a: i32 = *ref_a;
	println!("{}", "ok!");
    // task 4..end..

    // task 5
    	let a = 2;
	let ref_ref_a = &&a;
	
	do_nothing(**ref_ref_a); // fix this line
    // task 5..end..

    // task 6
    let v = vec![&1,&2,&3];
	
	let result = v.clone();
	do_nothing(result);
    // task 6..end..

    // task 7
    let v = &vec![&1, &2, &3];
	
	let c: Vec<&i32> = v.clone();
	do_nothing(c);
    // task 7..end..

    // task 8
    let hs: &HashSet<&i32> = &HashSet::from([&1, &2, &3]);

    let owned_hs =hs.clone().into_iter().copied().collect();
    do_nothing(owned_hs);
    // task 8..end..

    // task 9
    let t = &(10, 20);
	
	let c: (i32, i32) = *t; // edit this
	do_nothing(c);
    // task 9..end..

    // task 10
    let hs: &&HashSet<&i32> = &&HashSet::from([&1, &2, &3]);

    let owned_hs: HashSet<i32> = (*hs).clone().into_iter().copied().collect();
    do_nothing(owned_hs);
    // task 10..end..
}

// task 5 Copy types and nested references
fn do_nothing(_a: i32) {

}
// task 5..end..

// task 6 A clone on Vec<&i32>
fn do_nothing(_v: Vec<&i32>) {

}
// task 6..end..

// task 7 A clone on &Vec<&i32>
fn do_nothing(_v: Vec<&i32>) {

}
// task 7..end..

// task 8 Exercise: convert &Set<&i32> to Set<i32>
fn do_nothing(_hs: HashSet<i32>) {

}
// task 8..end..

// task 9 A clone on &tuple
fn do_nothing(_v: (i32, i32)) {

}
// task 9..end..