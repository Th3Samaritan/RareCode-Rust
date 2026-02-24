fn main() {
    // task 1 iter() autoderef
    let v = vec![1,2,3];
    
    let cv: Vec<&i32> = (&v).iter().collect();
    println!("{:?}", cv);
    // task 1..end..

    // task 2 iter() autoderefs an arbitrary amount of outside references
     let v = &&&&&&&&&&&vec![1,2,3];
    
    let cv: Vec<&i32> = (&&&&&&&&&&&&&&&&&&v).iter().collect();
    println!("{:?}", cv);
    // task 2..end..

    // task 3 iter() “adds” a reference to the inner type
    let v = vec![&&1,&&2,&&3];
    
    let cv: Vec<&&&i32> = v.iter().collect();
    println!("{:?}", cv);
    // task 3..end..

    // task 4 Exercise: iter() 1
    let v = vec![&1,&2,&3];
    
    let cv: Vec<&&i32> = (&v).iter().collect();
    println!("{:?}", cv);
    // task 4..end..

    // task 5 into_iter() “preserves” the number of inner references
       let v = vec![&&&1,&&&2,&&&3];
    
    let cv: Vec<&&&i32> = v.into_iter().collect();
    println!("{:?}", cv);
    // task 5..end..

    // task 6 into_iter() “absorbs” the outer reference
     let v = &vec![1,2,3];
    
    let cv: Vec<&i32> = v.into_iter().collect();
    println!("{:?}", cv);
    // task 6..end..

    // task 7 &v.into_iter() vs (&v).into_iter()
     let v = vec![1,2,3];
    
    let cv: Vec<&i32> = (&v).into_iter().collect();
    println!("{:?}", cv);
    // task 7..end..

    // task 8 into_iter() autoderefs nested outer references
    let v = vec![1,2,3];
    
    let cv: Vec<&i32> = (&&&&&&&&&&&&&v).into_iter().collect();
    println!("{:?}", cv);
    // task 8..end..

    // task 9 Exercise: converting a vector with outer and inner references
    	let v = vec![&&1,&&2,&&3];
		
		let _cv: Vec<&&&i32> = (&&&&&v).into_iter().collect();
        // task 9..end..

        // task 10 Exercise: iter() 2
        let v = vec![&&1,&&2,&&3];
		
		let _cv: Vec<&&&i32> = v.iter().collect();
		println!("{}", "success!");
        // task 10..end..

        // task 11 Exercise: iter() multiple references
        let v = &vec![&&1,&&2,&&3];
		
		let _cv: Vec<&&&i32> = v.iter().collect();
		println!("{}", "success!");
        // task 11..end..

        // task 12 Exercise: into_iter() 1
        	let v = &vec![&&1,&&2,&&3];
		
		let _cv: Vec<&&&i32> = v.into_iter().collect();
		println!("{}", "success!");
        // task 12..end..

        // task 13 Exercise: into_iter() 2
        let v = &&&vec![&&1,&&2,&&3];
		
		let _cv: Vec<&&&i32> = v.into_iter().collect();
		println!("{}", "success!");
        // task 13..end..

        // task 14 Exercise: iter() 4
        let v = vec![&1,&2,&3];
		
		let _cv: Vec<&&i32> = v.iter().collect();
		println!("{}", "success!");
        // task 14..end..

        // task 15 Exercise: iter() 5
        let v = &&vec![&1,&2,&3];
		
		let _cv: Vec<&&i32> = v.iter().collect();
		println!("{}", "success!");
        // task 15..end..
}
