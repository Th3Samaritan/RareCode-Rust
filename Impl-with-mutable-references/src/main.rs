// task 1 Inc Single Field
#[derive(Debug)]
pub struct SingleValue {
   pub x: u32,
}


impl SingleValue {
    pub fn inc_by(&mut self, y: u32) {
        self.x += y;
    }
}
// task 1..end..

// task 2 Exercise: Mirror Point
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
  pub fn mirror(&mut self)  {
        self.y = -self.y
}
}
// task 2..end..

// task 3 Exercise: Translate Point
#[derive(Debug, Clone, Copy)]
pub struct Point {
   pub x: i32,
   pub y: i32,
}
#[derive (Debug, Clone, Copy)]
pub enum Direction {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

impl Point {
    pub fn translate(&mut self, direction: Direction) {
        match direction{
        Direction::Up(amount) => {
            self.y += i32::from(amount);
        }
        Direction::Down(amount) => {
            self.y -= i32::from(amount);
        }
        Direction::Left(amount) => {
            self.x -= i32::from(amount);
            }
        Direction::Right(amount) => { 
            self.x += i32::from(amount);
        }
        }
    }
}
// task 3..end..

// task 4
#[derive(Debug)]
pub struct Message {
  pub text: String
}

impl Message {
    pub fn add_to_message(&mut self, s: &str){
        self.text.push_str(s);
    }
}
// task 4..end..

// task 5 Exercise: Vector values x2
#[derive(Debug)]
pub struct Holder {
   pub v: Vec<i32>,
}

impl Holder {
    pub fn double(&mut self){
        for e in self.v.iter_mut(){
            *e = *e * 2;
        }
    }
}
// task 5..end..

// task 6 Exercise: Remove odd values
#[derive(Debug)]
pub struct Holder {
   pub v: Vec<i32>,
}

impl Holder{
    pub fn remove_odd(&mut self) {
     self.v = self.v.iter().copied().filter(|x| x % 2 == 0 ).collect();
    }
}
// task 6..end..

// task 7 Exercise: Remove matching enums
#[derive(Debug)]
pub struct MyStyles {
    pub styles: HashSet<Style>
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Style {
    Classic,
    Modern,
    Hipster,
    Trendy
}

impl MyStyles {
    pub fn remove_style(&mut self, style: Style) {
        self.styles.remove(&style);
    }
}
// task 7..end..

// task 8 Exercise: Flip enum
#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn invert(&mut self) {
        match self {
        Color::Black => *self = Color::White,
        Color::White => *self = Color::Black,
        }
    }
}
// task 8..end..

fn main() {
    // task 1 Exercise: Inc Single Field
    let mut  s = SingleValue { x: 10 };
    s.inc_by(2);
    println!("{:?}", s); 
    // task 1..end..

    // task 2 Exercise: Mirror Point
    let mut p = Point { x: 10.0, y: -2.0 };
    p.mirror();
    println!("{:?}", p);
    // task 2..end..

    // task 3 Exercise: Translate Point
     let mut p = Point { x: 10, y: -2 };
    let d = Direction::Up(2);
    p.translate(d);
    p.translate(d);
    println!("{:?}", p);
    // task 3..end..

    // task 4 Exercise: Concat to Own String
     let mut m = Message { text: "Rare".to_string() };
    m.add_to_message("Code");
    println!("{:?}", m); 
    // task 4..end..

    // task 5 Exercise: Vector values x2
    let mut h = Holder { v: vec![1, 2, 3] };
    h.double();
    println!("{:?}", h);
    // task 5..end..

    // task 6 Exercise: Remove odd values
    let mut h = Holder { v: vec![1, 2, 3] };
    h.remove_odd();
    println!("{:?}", h);
    // task 6..end..

    // task 7 Exercise: Remove matching enums
    let mut m = MyStyles { styles: HashSet::from([
        Style::Classic,
        Style::Modern,
        Style::Hipster,
        Style::Trendy,
    ]) };
    
    let style = Style::Hipster;
    m.remove_style(style);
    println!("{:?}", m);
    // task 7..end..

    // task 8 Exercise: Flip enum
    let mut c = Color::Black;
    c.invert();
    println!("{:?}", c);
    // task 8..end..
}