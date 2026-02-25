fn main() {
    //  task 1 Vec<i32> to Vec<&i32>
    let v: Vec<i32> = vec![1,2,3];
    let _vc: Vec<&i32> = (&v).clone().iter().collect();
    // task 1..end..

    // task 2 Vec<i32> to &Vec<&i32>
    let v: Vec<i32> = vec![1,2,3];
    let _vc: &Vec<&i32> = &v.clone().iter().collect();
    // task 2..end..

    // task 3 &Vec<i32> to Vec<i32>
    let v: &Vec<i32> = &vec![1,2,3];
    let _vc: Vec<i32> = v.iter().copied().collect();
    // task 3..end..

    // task 4 &Vec<i32> to Vec<&i32>
    let v: &Vec<i32> = &vec![1,2,3];
    let _vc: Vec<&i32> = (&v).iter().clone().collect();
    // task 4..end..

    // task 5 &Vec<i32> to &Vec<&i32>
    let v: &Vec<i32> = &vec![1,2,3];
    let _vc: &Vec<&i32> = &v.iter().clone().collect();
    // task 5..end..

    // task 6 Vec<i32> to &Vec<i32>
    let v: Vec<i32> = vec![1,2,3];
    let _vc: &Vec<i32> = &v.iter().clone().copied().collect();
    // task 6..end..

    // task 7 Vec<&i32> to Vec<i32>
    let v: Vec<&i32> = vec![&1,&2,&3];
    let _vc: Vec<i32> = v.into_iter().copied().clone().collect(); 
    // task 7..end..

    // task 8 Vec<&i32> to &Vec<i32>
     let v: Vec<&i32> = vec![&1,&2,&3];
    let _vc: &Vec<i32> = &v.into_iter().clone().copied().collect();
    // task 8..end..

    // task 9 Vec<&i32> to &Vec<&32>
    let v: Vec<&i32> = vec![&1,&2,&3];
    let _vc: &Vec<&i32> = &v.iter().clone().copied().collect();
    // task 9..end..

    // task 10 &Vec<&i32> to Vec<i32>
    let v: &Vec<&i32> = &vec![&1,&2,&3];
    let _v: Vec<i32> = v.iter().clone().copied().copied().collect();
    // task 10..end..

    // task 11 &Vec<&i32> to Vec<&i32>
     let v: &Vec<&i32> = &vec![&1,&2,&3];
    let _v: Vec<&i32> = (&v).iter().clone().copied().collect();
    // task 11..end..

    // task 12 &Vec<&i32> to &Vec<i32>
    let v12: &Vec<&i32> = &vec![&1,&2,&3];
    let _v12: &Vec<i32> = &v12.into_iter().clone().copied().copied().collect();
    // task 12..end..
}