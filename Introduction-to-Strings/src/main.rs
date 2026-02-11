fn main() {
    // task 1
    let v = vec!['a', 'b', 'c', 'd', 'a'];
    let c: char = 'a';
    let result = count_char(&v, c);
    println!("{}", result);
    // task 1..end..

    // task 2
     let v_char = vec!['R', 'a', 'r', 'e', 'C', 'o', 'd', 'e'];
    
    let result: String = convert_to_string(v_char);
    println!("{}", result);
    // task 2..end..

    // task 3 .chars()
    let s = String::from("Hello, RareCode!");

    for e in s.chars() {
        println!("{}", e);
    }
      println!("{}", s);
    // task 3..end..

    // task 5 String Length
    let s1 = vec!['a'].into_iter().collect::<String>();
    let s2 = vec!['ǎ'].into_iter().collect::<String>();
    let s3 = vec!['आ'].into_iter().collect::<String>();
    let s4 = vec!['第'].into_iter().collect::<String>();
    let s5 = vec!['😎'].into_iter().collect::<String>();
    
    println!("a has len {}", s1.len());
    println!("ǎ has len {}", s2.len());
    println!("आ has len {}", s3.len());
    println!("第 has len {}", s4.len());
    println!("😎 has len {}", s5.len());
    // task 5..end..

    // task 6 String to bytes
    println!("----");
    println!("{}", s1.bytes().count());
    println!("{}", s2.bytes().count());
    println!("{}", s3.bytes().count());
    println!("{}", s4.bytes().count());
    println!("{}", s5.bytes().count());
    // task 6..end..

    // 7 Slicing a String
    let s1 = vec!['a', 'b'].into_iter().collect::<String>();
    let s2 = vec!['ǎ', 'b'].into_iter().collect::<String>();

    println!("a len {}", s1.len());
    println!("ǎ len {}", s2.len());

    let slice_1 = &s1[..2];
    let slice_2 = &s2[..3];

    println!("slice_1: {:?}", slice_1);
    println!("slice_2: {:?}", slice_2);
    // task 7..end..

    // task 8 Slice of a String
    
	let slice_of_string1: &str = &s[..4];
	let slice_of_string2: &str = &s;

	println!("{}", slice_of_string1);
	println!("{}", slice_of_string2);
    // task 8..end..

    // task 9 A String is not a copy type, but a String slice is
     let string_slice: &str = "Hello, world!";
    let actual_string: String = vec!['a', 'b', 'c'].into_iter().collect();

    let copy1 = string_slice;
    println!("{}", string_slice);

    let copy2 = actual_string.clone();
    println!("{}", actual_string);

    // use copies to avoid compiler warning
    println!("{} {}", copy1, copy2);
    // task 9..end..

    // task 10 String from slice
    // vector
	let a = [1, 2, 3];
	let slice_1 = &a[..];
	let _v = Vec::from(slice_1);

	// string
	let slice_2: &str = "hello RareCode";
	let s = String::from(slice_2);
	
	println!("{}", s);
    // task 10..end..

    // task 11 Slice into String
    let a = [1, 2, 3];
	let slice_1 = &a[..];
	let _v: Vec<i32> = slice_1.into();

	let slice_2 = "hello RareCode";
	let _s: String = slice_2.into();
	
	let _s2: String = "hello RareCode".into();
    // task 11..end..

    // task 12 Exercise: number of chars in a string
     let s: String = "Hello, world!".into();

    let result = count_characters(&s, 'o');
    println! {"{}", result};
    // task 12..end..

    // task 13 Exercise: replace spaces with underscores
    let s: String = "Time is an illusion. Lunchtime doubly so.".into();
    let result = replace_space(s);
    println! {"{}", result}; // "Time_is_an_illusion._Lunchtime_doubly_so."
    // task 13..end..

    // task 14 Exercise: LeetSpeak Substitution
      let s: String = "RareCode".into();

    let result = leet(s);
    println! {"{}", result};
    // task 14..end..

    // task 15 Exercise: Check if String is palindrome
    let s: String = "aǎअ家家अǎa".into();
    let result = is_palindrome(s);
    println! {"{}", result};
    // task 15..end..
}
// task 1
pub fn count_char(v: &[char], c: char) -> usize {
   v.iter().filter(|&&x| x==c).count()
}
// task 1..end..

// task 2 String from vec of chars
pub fn convert_to_string(v_char: Vec<char>) -> String {
 v_char.into_iter().collect() 
}
// task 2..end..

// task 3 String to vec of chars
pub fn convert_back(s: String) -> Vec<char> {
    s.chars().collect()
}
// task 3..end..

// task 12 Exercise: number of chars in a string

pub fn count_characters(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x==c).count()
}
// task 12..end..

// task 13 Exercise: replace spaces with underscores

pub fn replace_space(s: String) -> String {
    s.chars().map(|x| if x ==' ' {
        '_'} else {x}).collect()
}
// task 13..end..

// task 14

pub fn leet(s: String) -> String {
    s.chars().map(|x| if x=='a' {'4'} else if x=='e'{'3'} else{x}).collect()
}
// task 14..end..

// task 15 Exercise: Check if String is palindrome
pub fn is_palindrome(s: String) -> bool {
    let v: Vec<char> = s.chars().collect();
    for i in 0..v.len()/2{
        if v[i] != v[v.len() - 1- i]{
            return false;
        }
    }
    true
}
// task 15..end..