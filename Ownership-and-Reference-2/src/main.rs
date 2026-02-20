fn main() {
    // task 1 Fixed-size types
	let x = 2;
	let y = inc(x);
	println!("{}", y);
	println!("{}", x);
    // task 1..end..

    // task 2 Integer / Boolean Reassignment
    let x = true;
	let y = x;
	println!("{}", y);
	println!("{}", x);
    // task 2..end..

    // task 3
    let k = 5;
	let v = vec![4,5,6,7];
	
	let result = is_in_vector(&v, k);
	println!("{}", result);
	println!("{}", k);
	println!("{:?}", &v);
    // task 3..end..

    // task 4
    let k = 5;
	let v = vec![4,5,6,7];
	let idx = 1;
	
	let result = k_is_at_idx( &v,k, idx);
	println!("{}", result);
	println!("{}", k);
	println!("{}", idx);
	println!("{:?}", &v);
    // task 4..end..

    // task 5
    let v = vec![1,2,3];
	let k = 3;
	
	let result = find_idx_of(&v, k);
	println!("{}", result);
	println!("{}", k);
	println!("{:?}", &v);
    // task 5..end..

    // task 6
    let v1 = vec![1,2,3,4];
	let v2 = vec![2,2,3,4]; 
	
	let result = first_common_index(&v1, &v2);
	println!("{}", result);
	println!("{:?}", &v1);
	println!("{:?}", &v2);
    // task 6..end..

    // task 7
    let v = vec![1,2,3];
	
	let result = append_sum(&v);
	
	println!("{:?}", &v);
	println!("{:?}", &result);
    // task 7..end..

    // task 8 Increment Vector Elements
    let v = vec![1,2,3];
	let a = 2;
	
	let result = increment_by(&v, a);
	
	println!("{}", a);
	println!("{:?}", &v);
	println!("{:?}", &result);
    // task 8..end..

    // task 9
    let v = vec![1,2,3];
	
	let result = remove_max(&v);
	
	println!("{:?}", &v);
	println!("{:?}", &result);
    // task 9..end..
    
    // task 10
    let v = vec![1,2,3];
	let filter_even = true;
	
	let result = filter_even_odd(&v, filter_even);
	println!("{:?}", &v);
	println!("{}", filter_even);
	println!("{:?}", &result);
    // task 10..end..

}

// task 1 Fixed-size types
pub fn inc(x: i32) -> i32 {
    x + 1
} 
// task 1..end..

// task 3 Element Containment Check
pub fn is_in_vector(v: &Vec<i32>, k:i32) -> bool {
	for i in 0..v.len(){
        if v[i] == k {
            return true;
        }
    }false
} 
// task 3..end..

// task 4 Check Element at Specific Index
pub fn k_is_at_idx(v:&Vec<i32>, k:i32, idx:usize) -> bool {
    if idx >= v.len(){
        return false;
    }
        if v[idx] == k {
            return true;
        }
false
} 
// task 4..end..

// task 5 Find Index of Element
pub fn find_idx_of (v: &Vec<i32>, k:i32)->usize {
    for i in 0..v.len() {
        if v[i]%k==0 {
            return i;
        }
    }v.len()
}
// task 5..end..

// task 6 Find First Common Element Index
pub fn first_common_index(v1: &Vec<i32>, v2: &Vec<i32>) -> usize {
    if v1.len() == 0{
        return 0;
    } 
    if v2.len() == 0 {
        return v1.len()
    }
    let mut end = v1.len();
        if v2.len() < v1.len() {
            end = v2.len();
        }
    
    for i in 0..end {
        if v1[i] == v2[i] {
            return i;
        }
    }
    v1.len()
}
// task 6..end..

// task 7 References can be cloned
pub fn append_sum(v: &Vec<i32>) -> Vec<i32> {
	let mut sum = 0;
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
    let mut my_v = v.clone(); 
	my_v.push(sum);
	my_v
} 
// task 7..end..

// task 8 Increment Vector Elements
pub fn increment_by(v:&Vec<i32>, a:i32)->Vec<i32>{
    let mut y = v.clone();
    for i in 0..y.len(){
       y[i] = y[i]+a
    }
    y
}
// task 8..end..

// task 9 Remove Maximum Value
pub fn remove_max(v: &Vec<u32>) -> Vec<u32> 
{
  if v.len() == 0 {
        return vec![];
    }
	let mut max_val = v[0];
    let mut max_idx = 0;
    for i in 0..v.len(){
        if v[i]>max_val{
            max_val = v[i];
            max_idx = i;
        }
    }
    let mut result = v.clone();
    result.remove(max_idx);
  result
} 
// task 9..end..

// task 10 Filter Even or Odd Numbers
pub fn filter_even_odd(v: &Vec<i32>, filter_even:bool)->Vec<i32>{
    let mut y = vec![];
    for i in 0..v.len(){
         if filter_even == true {
          if v[i] % 2 !=0 {
            y.push(v[i]);
        }
    } else {
        if v[i] % 2 == 0 {
            y.push(v[i]);
        }
    }
       
    }
    y
}
// task 10..end..