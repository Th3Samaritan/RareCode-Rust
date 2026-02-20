// task 1 where ::from() is from
#[derive(Debug)]
pub struct MyStruct {
    pub x: u32,
}

impl MyStruct {
    pub fn from(x: u32) -> MyStruct {
        MyStruct { x: x }
    }
}
// task 1..end..

// task 2 Exercise: Point factory
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn from(x: i32, y: i32) -> Point {
        Point {
            x: x, y: y
        }
    }
}
// task 2..end..

// task 3
#[derive(Debug, PartialEq)]
pub struct Stack {
    items: Vec<i32>,
}

impl Stack {
    pub fn new() -> Stack  {
        Stack { items: vec![]}
    }
    pub fn from(slice: &[i32]) -> Stack {
        Stack {
           items: Vec::from(slice)
        }
    }
    pub fn push(&mut self, x: i32) {
    self.items.push(x);
}
pub fn peek(&self) -> Option<i32> {
    if self.items.is_empty() {
        None
    } else {
        Some(self.items[self.items.len() - 1])
        }
}
pub fn pop(&mut self)-> Option<i32>{
    self.items.pop()
    }
pub fn len(&self) -> usize {
    self.items.len()
}
}
// task 3..end..

// task 4
pub struct MyStruct {
   pub  x: i32,
}

impl MyStruct {
    const MEANING_OF_LIFE: u32 = 42;
    const  MAX_U8_AS_U32:u32 = 255;
}
// task 4..end..

// task 5
pub struct Water {
 pub volume: u32,
}

impl Water {
    const FREEZING: i32 = 0;
    const BOILING: i32 = 100;
}
// task 5..end..

fn main() {
    // task 1 where ::from() is from
    let ms = MyStruct::from(3);
    assert!(ms.x == 3);
    println!("{:?}", ms);
    // task 1..end..

    // task 2 Exercise: Point factory
     let p = Point::from(3, -4);

    assert!(p.x == 3);
    assert!(p.y == -4);
    println!("{:?}", p);
    // task 2..end..

    // task 3 Stack Data Structure
     let mut stack1 = Stack::new();
    let stack2 = Stack::from(&[1, 2, 3]);

    stack1.push(1);
    stack1.push(2);
    stack1.push(3);

    println!("{}", stack1 == stack2);
    stack1.pop();
    println!("{}", stack1 == stack2);
    println!("{:?}", stack2.peek());
    // task 3..end..

    // task 4 Associated Constants
     let meaning = MyStruct::MEANING_OF_LIFE;
    let maxU8: u32 = MyStruct::MAX_U8_AS_U32;

    assert!(meaning == 42);
    assert!(maxU8 == 255);
    // task 4..end..

    // task 5 Exercise: Associated Constants
     let t0 = Water::FREEZING;
    let t1 = Water::BOILING;

    assert!(t0 == 0);
    assert!(t1 == 100);
    // task 5..end..
}