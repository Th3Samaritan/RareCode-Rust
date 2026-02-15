fn main() {
    // task 1 introduction to to_string()
    let msg = "😎😎😎".to_string();

    let result = num_chars(msg);
    println!("{}", result);
    // task 1..end..

    // task 2 to_string() with integers
    let number = 222.to_string();

    let result = num_chars(number);
    println!("{}", result);
    // task 2..end..

    // task 3 to_string() with bool
    let b = false;
    let result = bool_to_string(b);
    println!("{}", result);
    // task 3..end..

    // task 4 to_string() with char
    let ch = 'अ';
    let result = char_to_string(ch);
    println!("{}", result);
    // task 4..end..

    // task 5 to_string() also works with references
     let n = 12345;
    let result = ref_num_to_string(&n);
    println!("{}", result);
    // task 5..end..
}

// task 1 introduction to to_string()
pub fn num_chars(s: String) -> usize {
    s.chars().count()
}
// task 1..end..

// task 2 to_string() with integers
pub fn num_chars(s: String) -> usize {
    s.chars().count()
}
// task 2..end..

// task 3 to_string() with bool
pub fn bool_to_string(b: bool) -> String {
    if b == true {
        return true.to_string();
    } else {
        false.to_string()
    }
}
// task 3..end..

// task 4 to_string() with char
pub fn char_to_string(ch: char) -> String {
    ch.to_string()
}
// task 4..end..

// task 5 to_string() also works with references
pub fn ref_num_to_string(x: &i32)->String{
    x.to_string()
}
// task 5..end..