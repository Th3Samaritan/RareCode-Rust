// task 1 Attaching a method to a struct
pub struct Square {
   pub side: f32,
}

impl Square{
   pub fn area(&self)->f32 {
        self.side * self.side
    }
}
// task 1..end..

// task 3 Impl for enum
#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
   pub fn interpret(&self) -> String {
        match self {
            Color::Black => "dark mode".to_string(),
            Color::White => "light mode".to_string(), 
        }
    }
    
   pub fn flip(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
// task 3..end..

// task 4 Exercise: Water State
pub struct Water {
   pub volume: u32,
   pub temperature: i32,
}

impl Water {
    pub fn is_boiling(&self)->bool{
        if self.temperature >= 100 {
            return true;
        }else {
            false
        }
    }
    pub fn is_frozen(&self)->bool{
        if self.temperature <= 0 {
            return true;
        }else {
            false
        }
    }
    pub fn empty(&self)->bool{
        if self.volume == 0{
            return true;
        }else{
            false
        }
    }
}
// task 4..end..

// task 5
pub enum Coord {
    Latitude(f32),
    Longitude(f32), 
}

impl Coord { 
pub fn get_inner(&self)->f32{
    match self {
        Coord::Latitude(x) => *x,
        Coord::Longitude(x) => *x,
        }
    }
}

// task 5..end..

// task 6
pub enum MyOption {
    Some(i32),
    None,
}

impl MyOption{
    pub fn is_none(&self)->bool{
       if let MyOption::None = self {
            return true;
        }
        false
    }
    pub fn is_some(&self)->bool {
        match self{
            MyOption::Some(_) => true,
            MyOption::None => false, 
        }
    }
    pub fn unwrap(&self)->i32{
        match self {
            MyOption::Some(val) => *val,
            MyOption::None => panic!("message"),
        }
    }
}
// task 6..end..

// task 7
#[derive(Debug, Clone, Copy)]
pub struct A {
   pub  x: u32,
}

impl A {
   pub fn clone_myself(&self) -> A {
        self.clone()
    }
}
// task 7..end..

// task 8
#[derive(Debug, Clone)]
pub struct Single {
    pub x: u32,
}

impl Single {
   pub fn inc(&self, value: u32) -> Single {
        Single { x: self.x + value }
    }
}
// task 8..end..

// task 9
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
   pub fn point_add(&self, p2: &Point) -> Point {
        let p1x = if self.x.is_nan() { 0.0 } else { self.x };
        let p2x = if p2.x.is_nan() { 0.0 } else { p2.x };
         let p1y = if self.y.is_nan() { 0.0 } else { self.y };
        let p2y = if p2.y.is_nan() { 0.0 } else { p2.y };

        Point {
            x: p1x + p2x,
            y: p1y + p2y,
        }
    }
}

// task 9..end..

// task 10
#[derive(Debug)]
pub struct Message {
    pub text: String,
}

impl Message {
    pub fn combine_message(&self, other: &Message)-> Message {
         let mut new_text = self.text.clone();
        new_text.push_str(&other.text);
        Message { text: new_text }
    }
}

// task 10..end..

fn main() {
    // task 1 Exercise: Area of a Square
    let c = Square { side: 1.0 };
    let a = c.area();
    
    println!("{}", a);
    // task 1..end..

    // task 3
    let color = Color::Black;
    let result = color.interpret();
    let opposite: Color = color.flip();

    println!("{} {:?}", result, opposite);
    // task 3..end..

    // task 4 Exercise: Water State
    let w = Water { volume: 1, temperature: 50 };

    let ice = w.is_frozen();
    let steam = w.is_boiling();
    let empty = w.empty();

    println!("{} {} {}", ice, steam, empty);
    // task 4..end..

    // task 5 Exercise: enum inner wrapped value
     let c = Coord::Latitude(0.9);
    let result = c.get_inner();

    println!("{}", result);
    // task 5..end..

    // task 6 Exercise: My Option
    let o1 = MyOption::Some(3);
    let o2 = MyOption::None;

    println!("{}", o1.is_some());
    println!("{}", o1.unwrap());
    println!("{}", o2.is_none());
    // task 6..end..

    // task 7 Returning Clone of Self
    let a = A { x: 1 };
    let result = a.clone_myself();
    println!("{:?}", result);
    // task 7..end..

    // task 8 Self with new value
    let s = Single { x: 5 };
    let new_s = s.inc(2);
    println!("{:?}", new_s);
    // task 8..end..

    // task 9 Exercise: Point Add
     let p1 = Point { x: 5.0, y: f32::NAN };
    let p2 = Point { x: 1.0, y: 1.0 };
    let result = p1.point_add(&p2);
    println!("{:?}", result);
    // task 9..end..

    // task 10 Exercise: Extend Message
    let m1 = Message { text: "Rare".to_string() };
    let m2 = Message { text: "Code".to_string() };
    let result = m1.combine_message(&m2);
    println!("{:?}", result);
    // task 10..end..
}
