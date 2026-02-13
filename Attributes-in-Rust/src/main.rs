// task 1 Cloning an enum
#[derive(Clone)]
enum TaxiType {
    Car,
    // we'll add other types later
}
// task 1..end..

// task 2 Printing an Enum
#[derive(Debug)]
enum Pet {
    Dog,
    Cat,
}
// task 2..end..

// task 3 Exercise: Compilation
#[derive(Debug)]
enum Foo {
    Foo
}
#[derive(Clone)]
enum Bar {
    Bar
}
// task 3..end..

// task 4 Cloneable and Printable
#[derive(Clone, Debug)]
enum Foo {
    Foo,
}
// task 4..end..

// task 5 Enum Copy Types
#[derive(Clone, Copy)]
enum Pets {
    Dog,
    Cat
}
// task 5..end..

// task 6 Exercise: Direction Changer
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}
// task 6..end..
fn main() {
    let taxi_type = TaxiType::Car;
    
    let _taxi_type_clone = taxi_type.clone();
    // task 1..end..

    // task 2 Printing an Enum
    let pet = Pet::Dog;
    println!("{:?}", pet);

    let pet = Pet::Cat;
    println!("{:?}", pet);
    // task 2..end..

    // task 3 Exercise: Compilation
    let f = Foo::Foo;
    let b = Bar::Bar;
    
    println!("{:?}", f);
    let _b_clone = b.clone();
    // task 3..end..

    // task 4 Cloneable and Printable
    let foo = Foo::Foo;

    let foo_clone = foo.clone();
    println!("{:?}", foo_clone);
    // task 4..end..

    // task 5 Enum Copy Types
     let pet = Pets::Dog;
    let _pet2 = pet;
    
    match pet {
        Pets::Dog => println!("woof!"),
        Pets::Cat => println!("meow!"),
    };
    
    let pet = Pets::Cat;
    let _pet2 = pet;
    
    match pet {
        Pets::Dog => println!("woof!"),
        Pets::Cat => println!("meow!"),
    }
    // task 5..end..

    // task 6 Exercise: Direction Changer
    let mut direction = Direction::Left;

    print_direction(direction);

    change_direction(&mut direction);

    print_direction(direction);
    // task 6..end..
}

// task 6 Exercise: Direction Changer
pub fn print_direction(d: Direction) {
    println!("{:?}", d);
}

pub fn change_direction(d: &mut Direction) {
    match d {
        Direction::Left => *d = Direction::Right,
        Direction::Right => *d = Direction::Left,
    };
}
// task 6..end..