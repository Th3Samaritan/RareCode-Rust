fn main() {
    // task 1 push
    let mut s: String = "我要茶".into();

    push_lt_5(&mut s);
    println!("{}", s);
    // task 1..end..

    // task 2 Exercise: push until
    let mut s = "hello".into();
    pad_right_to_10(&mut s);
    println!("{}|", s); 
    // task 2..end..

    // task 3 Concatenating Strings
     let result = very_happy(3);
    println!("{}", result);
    // task 3..end..

    // task 4 Exercise: Palindromize
     let mut input = String::from("123");
    palindromize(&mut input);
    println!("{}", input);
    // task 4..end..

    // task 5 
     let s1 = "hello";
    let s2 = "你好";
    let result = ordered_cat(s1, s2);
    println!("{}", result);
    // task 5..end..

    // task 6 Exercise: Print Grid
    let a = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let result = to_grid(&a);
    println!("{}", result);
    // task 6..end..
}
// task 1 push
pub fn push_lt_5(s: &mut String) {
    if s.chars().count() < 5 {
        s.push('!')
    }
}
// task 1..end..

// task 2 Exercise: push until
pub fn pad_right_to_10(s: &mut String) {
    while s.chars().count() < 10 {
        s.push(' ');
    }
}
// task 2..end..

// task 3 Concatenating Strings
pub fn very_happy(n: usize) -> String {
    let mut s = String::from("I'm ");
    for _ in 0..n {
        s.push_str("very ");
    }
    s.push_str("happy!");
    s
} 
// task 3..end..

// task 4 Exercise: Palindromize
pub fn palindromize(s: &mut String) {
    s.push_str(&s.chars().rev().collect::<String>());
}
// task 4..end..

// task 5 Exercise: Short Before Long
pub fn ordered_cat(s1: &str, s2: &str) -> String {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    if len1 <= len2{
        let mut result = String::from(s1);
        result.push_str(s2);
        result
    } else {
        let mut result = String::from(s2);
        result.push_str(s1);
        result
    }
}
// task 5..end..

// task 6 Exercise: Print Grid
pub fn to_grid(a: &[char; 9]) -> String {
    let mut grid = String::new();
    for i in 0..3 {
        let mut row = String::from("|");
        for j in 0..3 {
            let index = i * 3 + j;
            row.push(a[index]);
            row.push('|');
        }
        row.push_str("\n");
        grid.push_str(&row);
    }
    grid
}
// task 6..end..