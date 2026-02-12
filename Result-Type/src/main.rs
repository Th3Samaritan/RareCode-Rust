fn main() {
    // task 1 introduction to Result type
    let num = "-999";
    let result = num.parse::<i8>();

    if result.is_ok() {
        println!("Parsed number: {}", result.unwrap());
    } else {
        println!("{}", result.err().unwrap());
    }
    // task 1..end..

    // task 2 Exercise: Parse vector
     let v = ["1", "2", "h", "3"];
    let result = parse_or_remove(&v);
    println!("{:?}", result);
    // task 2..end..

    // task 3 Downcasting numbers
      let nums: Vec<u16> = vec![1, 2, 999];
    let result = downcast(&nums);
    println!("{:?}", result);
    // task 3..end..

    // task 4 Detecting which error happened
    let num = "-999";
    let res= num.parse::<i8>();
    if res.is_err() {
        let error = res.err().unwrap();
        if *error.kind() == std::num::IntErrorKind::PosOverflow {
            println!("overflow detected");
        }

        else if *error.kind() == std::num::IntErrorKind::NegOverflow {
            println!("negative overflow detected");
        }
        
        else if *error.kind() == std::num::IntErrorKind::InvalidDigit {
            println!("invalid digit");
        } 
        else if *error.kind() == 
        std::num::IntErrorKind::Empty{
            println!("empty string");
        }
        else {
            println!("other error: {:?}", *error.kind());
        }
        
    } else {
        println!("Parsed successfully: {}", res.unwrap());
    }
    // task 4..end..
}

// task 2 Exercise: Parse vector
pub fn parse_or_remove(v: &[&str]) -> Vec<i16> {
    let mut result = Vec::new();
    for &s in v {
        let parsed = s.parse::<i16>();
        if parsed.is_ok(){
            result.push(parsed.unwrap());
        }
    }
    result
}
// task 2..end..

// task 3 Downcasting numbers

pub fn downcast(slice: &[u16]) -> Vec<u8> {
   let mut ret = Vec::new();
    for num in slice {
        let result = u8::try_from(*num);
        
        if result.is_ok() {
            ret.push(result.unwrap());
        }
    }
    ret
}
// task 3..end..