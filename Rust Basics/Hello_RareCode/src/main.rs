fn main (){
    println!("{}","Hello, world!");
}

// task 2 Declare a variable x and assign it the value 42. Return the value of x.
pub fn declare_variable_with_x() -> i32 {
    let x = 42;
    println!("{}", x);
}

// task 3 Increment X by 1 and return the value of X.
pub fn increment(x: i32) -> i32 {
    x + 1
}

//task 4 Double the value of X and return the new value.
pub fn double(x: i32) -> i32 {
    x * 2
}

//task 5 Negate the value of X and return the new value.

pub fn negate(x: i32) -> i32 {
    x * -1
}

// task 6 less than a 100

pub fn less_than_100(x: i32) -> bool {
    if x < 99 {
        return true;   
    }
    false
}

// task 7 Between 50 and 100
pub fn between_50_and_100(x: i32) -> bool {
    if x < 100 && x > 50 {
        return true;   
    }
    false
}

// task 8 Floor 100
pub fn floor100(x: i32) -> i32 {
    if x < 100 {
        return 100;
    }
    x
}

// task 9 is Divisible by
pub fn is_divisible_by(x: i32, y: i32) -> bool {
    x % y == 0
}

// task 10 Sum Function

pub fn sum(x: i32, y: i32) -> i32 {
	x + y
}

// task 11 Maximum Numbers
pub fn max(x:i32, y:i32) -> i32 {
    if x < y {
        return y;
    }
    x
}

// task 12 numbers greater than 20

pub fn x_and_y_greater_than_20(x:i32, y:i32)->bool {
    if x > 20 && y > 20 {
        return true;
    }
    false
}

// task 13 less than 10
pub fn x_or_y_less_than_10(x:i32, y:i32) -> bool {
    if x < 10 || y < 10 {
        return true;
    }
    false
}

// task 14 Divisibility Checker 

pub fn divisible_by(x: i32) -> i32 {
    if x % 2 == 0 {
        return 2;
    }
    else if x % 3 == 0 {
        return 3;
    }
    else if x % 5 == 0 {
        return 5;
    }
    else {
        return 0;
    }
} 

// task 15 calculator function
pub fn calculator(x: i32, y: i32, op: i32) -> i32 {
    if op == 0 {
        x + y
    }
    else if op == 1 {
            x - y
        }
    else if op == 2 {
            x * y
        }
    else if op == 3 {
            x / y
        }
    else {
        x % y
    }
} 