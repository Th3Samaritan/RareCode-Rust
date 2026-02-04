// task 1 Mutable Variables
fn main() {
    let mut x = 3;
    x = x + 1;
    println!("{}", x);
// task 1..end..

// task 2 for counter
for i in 0..6 {
		println!("{}", i);
	}
// task 2..end..

// task 3 Variable Range loop
let start = 0;
	let end = 5;
	for i in start..end {
		println!("{}", i);
	}
// task 3..end..
// task 4 loop counter with mut variable
let mut x = 1;
	for i in 0..3 {
		x = x + 1;
	}
	println!("{}", x);
// task 4..end..

// task 10 Countdown loop
let n = 10;
	
	for i in 0..n {
		let backward_i = n - i;
		println!("{}", backward_i);
	}
}
// task 5 Count even numbers

pub fn count_evens(start: u32, end: u32) -> u32 {
    let mut count = 0;
    for i in start..end {
        if i % 2 ==0 {
            count = count + 1
        }
    }
    count
} 

// task 5..end..

// task 6 Sum even numbers
pub fn sum_evens(start: u32, end: u32) -> u32 {
    let mut sum = 0;
    for i in start..end {
        if i % 2 == 0 {
            sum = sum + i
        }
    }
    sum
} 

// task 6..end..

// task 7 Product by addition

pub fn prod(x: u32, n: u32) -> u32 {
    let mut p = 0;
    // prefix i with _i to silence the compiler warning
    for _i in 0..n {
        p = p + x;
    }
    p
}
// task 7..end..

// task 8 Power Function

pub fn power(base: u32, exponent: u32) -> u32 {
	let mut acc = 1;
	// your code here
    for _i in 0..exponent {
        acc = acc * base 
    }
	acc
}
// task 8..end..

// task 9 Prime Number Checker


pub fn is_prime(x: u32) -> bool {
    // Handle edge cases: numbers less than 2 are not prime
    if x < 2 {
        return false;
    }
    
    for i in 2..(x/2+1) {
        if x % i == 0 {
            return false;
        }
    }
    
    true
}
// task 9..end..

// task 11 Largest Proper Divisor

pub fn largest_proper_divisor(x: u32) -> u32 {
  // loop down to find the largest proper divisor
  // your code here
  for i in (1..x).rev() {
        if x % i == 0 {
            return i;
        }
    }
    1
}
// task 11..end..

// task 12 Fibonacci Function

pub fn fibonacci(n: u32) -> u32 {
if n == 0 || n == 1 {
    return 1;
}
let mut prev = 1;
let mut curr = 1;

for _ in 2..=n {
    let next = curr + prev;
    prev = curr;
    curr = next;
}
curr
}
// task 12..end..

// task 13 Factorial Function

pub fn factorial(n: u32) -> u32 {
  if n==0 || n==1 {
    return 1;
  }
  let mut product = 1;
for i in 1..=n{
     product = product * i;
}
product
} 
// task 13..end..