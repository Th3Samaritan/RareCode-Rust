// task 1 Introduction to Structs
#[derive(Debug)]
struct Person {    
    name: String,
    age: u8,
}
// task 1..end..

// task 2 Accessing fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
// task 2..end..

// task 3 Exercise
pub struct Point {
    pub x: f32,
    pub y: f32,
}
// task 3..end..

// task 4 Returning a struct
#[derive(Debug)]
pub struct Account {
    pub account_number: u32,
    pub balance: u32,
    pub owner: String,
}
// task 4..end..

// task 5 Exercise: return area of rectangle
pub struct Rectangle {
    pub upper: (u32, u32),
    pub lower: (u32, u32),
}
// task 5..end..

// task 6 Exercise: Declare Struct
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
// task 6..end..

// task 7 Exercise: Pointwise Sum
#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
// task 7...end..

// task 8 Exercise: Set Smaller to Zero
#[derive(Debug)]
pub struct Pair {
    pub a: u32,
    pub b: u32,
}
// task 8..end..

// task 9 Exercise: Restaurant Rating
#[derive(Debug)]
pub struct Restaurant {
   pub name: String,
   pub stars: u8,
}
// task 9..end..

// task 10 Exercise: Struct Containing and Enum
#[derive(PartialEq)]
pub enum Status {
    NotStarted,
    InProgress,
    Done,
}

pub struct Task {
   pub status: Status,
    pub description: String
}
// task 10..end..
fn main() {
    // task 1 Introduction to Structs
    let person = Person {
        name: "Bob".to_string(),
        age: 50
    };
    
    println!("{:?}", person);
    // task 1..end..

    // task 2 Accessing fields
    let p = Point { x: 3.0, y: 4.0 };
    
    println!("x coordinate: {}", p.x);
    println!("y coordinate: {}", p.y);
    // task 2..end..

    // task 3 Exercise Exercise: Distance From Origin
     let point = Point {
        x: 5.0,
        y: 6.0,
    };
    
    let result = distance(point);
    println!("{}", result);
    // task 3..end..

    // task 4 Returning a struct
     let result = create_account(0, 0, "Bob");
    println!("{:?}", result);
    // task 4..end..

    // task 5 Exercise: return area of rectangle
    let rectangle = Rectangle {
        upper: (10, 12),
        lower: (4, 6),
    };
    
    let result = area(rectangle);
    println!("{}", result);
    // task 5..end..

    // task 6 Exercise: Declare Struct
    let p3d = Point3D {
        x: 7,
        y: 9,
        z: -1,
    };
    
    let result = is_first_octant(p3d);
    println!("{}", result);
    // task 6..end..

    // task 7 Exercise: Pointwise Sum
     let v = vec![Point { x: 3, y: 6}, Point { x: 7, y: 4 }];
    let result = pointwise_sum(v);
    println!("{:?}", result);
    // task 7..end..

    // task 8 Exercise: Set Smaller to Zero
     let mut v = vec![Pair { a: 3, b: 4 }, Pair { a: 10, b: 9 }, Pair { a: 14, b: 14 }];
    to_zero(&mut v);
    println!("{:?}", v);
    // task 8..end..

    // task 9 Exercise: Restaurant Rating
    let v = vec![
        Restaurant { name: "cookhouse".to_string(), stars: 4 },
        Restaurant { name: "infinitecoffee".to_string(), stars: 5 },
        Restaurant { name: "pastaden".to_string(), stars: 3 },
        Restaurant { name: "aliceandbobcafe".to_string(), stars: 2 },
    ];
    
    let result = at_least_n_stars(v, &4);
    println!("{:?}", result);
    // task 9..end..

    // task 10 Exercise: Struct Containing and Enum
    let tasks = vec![
        Task { status: Status::NotStarted, description: "clean the windows".to_string() },
        Task { status: Status::Done, description: "clean the floors".to_string() },
        Task { status: Status::InProgress, description: "clean the dishes".to_string() }
    ];
    
    let result = count_by_status(tasks, Status::InProgress);
    println!("{}", result);
    // task 10..end..
}

// task 3 Exercise: Distance From Origin
pub fn distance(p: Point) -> f32 {
    f32::sqrt(p.x * p.x + p.y * p.y)
}
// task 3..end..

// task 4 Returning a struct
pub fn create_account(account_number: u32, balance: u32, owner: &str) -> Account {
    Account {
        balance: balance,
        account_number: account_number,
        owner: owner.to_string()
    }
}
// task 4..end..

// task 5 Exercise: return area of rectangle
pub fn area(rectangle:Rectangle) -> u32 {
        let width = rectangle.upper.0 - rectangle.lower.0;
        let height = rectangle.upper.1 - rectangle.lower.1;
        width * height
}
// task 5..end..

// task 6 Exercise: Declare Struct
pub fn is_first_octant(p3d: Point3D) -> bool {
    p3d.x >= 0 && p3d.y >= 0 && p3d.z >= 0
}
// task 6..end..

// task 7 Exercise: Pointwise Sum
pub fn pointwise_sum(v: Vec<Point>)->Point{
    let mut x_sum = 0;
   let mut y_sum = 0;

   for p in v {
    x_sum += p.x;
    y_sum += p.y;
   }
   
   Point {
    x: x_sum,
    y: y_sum,
   }
} 
// task 7..end..

// task 8 Exercise: Set Smaller to Zero
pub fn to_zero(v: &mut Vec<Pair>) {
    for p in v {
        if p.a < p.b {
            p.a = 0;
        } else if p.b < p.a {
            p.b = 0;
        }

    }
}
// task 8..end..

// task 9 Exercise: Restaurant Rating
pub fn at_least_n_stars(v: Vec<Restaurant>, stars: &u8) -> Vec<Restaurant> {
    v.into_iter().filter(|r| {r.stars >= *stars}).collect()
}
// task 9..end..

// task 10 Exercise: Struct Containing and Enum
pub fn count_by_status(tasks: Vec<Task>, status: Status) -> usize {
    tasks.into_iter().filter(|task| task.status == status).count()
}
// task 10..end..