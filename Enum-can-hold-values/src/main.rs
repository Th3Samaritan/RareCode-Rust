// task 1 Enums can hold values
#[derive(Debug, Copy, Clone)]
pub enum Area {
    SquareFeet(u32),
    Acres(u32),
    Hectares(u32),
}
// task 1..end..

// task 2 Exercise: Convert to Cups
#[derive(Debug, Copy, Clone)]
pub enum ImperialVolume{
    Cups(u32),
    Pints(u32),
    Gallons(u32),
}
// task 2..end..

// task 4 Enum holding other enum
#[derive(Debug, Copy, Clone)]
pub enum Piece {
    Black,
    White,
}

#[derive(Debug, Copy, Clone)]
pub enum Square {
    Occupied(Piece),
    Empty,
}
// task 4..end..

// task 3 Options are Enums
#[derive(Debug, Copy, Clone)]
pub enum MyOption {
    Some(u32),
    None,
}
// task 3..end..

// task 5 Exercise: Pet Attributes
#[derive(Debug, Clone, Copy)]
pub enum Pet {
    Dog(DogType),
    Cat (CatType),
}
# [derive (Debug, Clone, Copy)]
pub enum DogType {
    Beagle,
    Poodle,
}
# [derive (Debug, Clone, Copy)]
pub enum CatType {
    Persian,
    Siamese,
}

// task 5..end..

// task 6 Results are also Enums
#[derive(Debug)]
pub enum MyResult {
    Ok(i8),
    Err(DivisionError)
}

#[derive(Debug)]
pub enum DivisionError {
    DivideByZero,
    Overflow,
}
// task 6..end..

// task 7 Exercise: Http Response Codes
#[derive(Debug)]
pub enum HttpResponse {
    Informational(u16),
    Successful(u16),
    Redirection(u16),
    BadRequest(u16),
    ServerError(u16),
    Invalid(u16),
}
// task 7..end..

fn main() {
    // task 1 Enums can hold values
    let a = Area::Acres(2);
    let result = to_square_feet(a);
    println!("{:?} is {:?}", a, result);
    // task 1..end..

    // task 2 Exercise: Convert to Cups
     let a = ImperialVolume::Pints(2);
    let result = to_cups(a);
    println!("{:?} is {:?}", a, result);
    // task 2..end..

    // task 3 Options are Enums
    let x = -10;
    let result = convert(x);
    println!("{:?}", result);
    
    let x = 0;
    let result = convert(x);
    println!("{:?}", result);
    // task 3..end..

    // task 4 Enum holding other enum
    let mut square = Square::Occupied(Piece::Black);
    println!("{:?}", square);
    
    flip_color(&mut square);
    println!("{:?}", square);
    
    make_empty(&mut square);
    println!("{:?}", square);
    // task 4..end..

    // task 5 Exercise: Pet Attributes
    let pet_1 = Pet::Dog(DogType::Beagle);
    let pet_2 = Pet::Cat(CatType::Siamese);
    let pet_3 = Pet::Dog(DogType::Poodle);
    let pet_4 = Pet::Cat(CatType::Persian);
    
    for pet in [pet_1, pet_2, pet_3, pet_4] {
        println!("{:?} {:?} {:?}", pet, sound(pet), color(pet));
    }

    // task 5..end..

    // task 6 Results are also Enums
    let numerator = 10;
    let denominator = 0;
    println!("{:?}", my_divide(numerator, denominator));
    
    let numerator = -128;
    let denominator = -1;
    println!("{:?}", my_divide(numerator, denominator));
    
    let numerator = 50;
    let denominator = -2;
    println!("{:?}", my_divide(numerator, denominator));
    // task 6..end..

    // task 7 Exercise: Http Response Codes
    let code = 100;
    println!("{:?}", code_to_response(code));
    let code = 200;
    println!("{:?}", code_to_response(code));
    let code = 300;
    println!("{:?}", code_to_response(code));
    let code = 400;
    println!("{:?}", code_to_response(code));
    let code = 500;
    println!("{:?}", code_to_response(code));
    let code = 99;
    println!("{:?}", code_to_response(code));
    let code = 600;
    println!("{:?}", code_to_response(code));

    // task 7..end..
}

 // task 1 Enums can hold values
pub fn to_square_feet(area: Area) -> Area {
    match area {
        Area::Hectares(x) => Area::SquareFeet(x * 107639),
        Area::Acres(x) =>
            Area::SquareFeet(x * 43560),
        Area::SquareFeet(x) =>
            Area::SquareFeet(x*1),
    }
}
// task 1..end..

// task 2 Exercise: Convert to Cups
pub fn to_cups(vol: ImperialVolume) -> ImperialVolume {
    match vol {
        ImperialVolume::Cups(x) =>
            ImperialVolume::Cups(x * 1),
        ImperialVolume::Pints(x) =>
            ImperialVolume::Cups(x * 2),
        ImperialVolume::Gallons(x) =>
            ImperialVolume::Cups(x * 16),
    }
}

// task 2..end..

// task 3 Options are Enums
pub fn convert(x: i32) -> MyOption {
        if x < 0 {
           MyOption::None}
           else{
                MyOption::Some(x as u32)
        }
    
}
// task 3..end..

// task 4 Enum holding other enum
pub fn flip_color(square: &mut Square) {
    match square {
        Square::Occupied(Piece::Black) => *square = Square::Occupied(Piece::White),
        Square::Occupied(Piece::White) =>
            *square = Square::Occupied(Piece::Black),
        Square::Empty => {}, // do nothing
    }
}

pub fn make_empty(square: &mut Square) {
       *square = Square::Empty;
}
// task 4..end..

// task 5 Exercise: Pet Attributes
pub fn sound(pet: Pet) -> String {
    match pet {
        Pet::Dog(_) => String::from("woof"),
        Pet::Cat(_) => String::from("meow"),
    }
}

pub fn color(pet: Pet) -> String {
    match pet {
        Pet::Dog(DogType::Beagle) => String::from("brown"),
        Pet::Dog(DogType::Poodle) =>
            String::from("white"),
        Pet::Cat(CatType::Persian) =>
            String::from("orange"),
        Pet::Cat(CatType::Siamese) =>
            String::from("gray"),
    }
}
// task 5..end..

// task 6 Results are also Enums
pub fn my_divide(numerator: i8, denominator: i8) -> MyResult {
     if denominator == 0 {
        return MyResult::Err(DivisionError::DivideByZero);
    }
    if numerator == i8::MIN && denominator == -1 {
        return MyResult::Err(DivisionError::Overflow);
    }
    MyResult::Ok(numerator / denominator)
}

// task 6..end..

// task 7 Exercise: Http Response Codes

pub fn code_to_response(code: u16) -> HttpResponse {
     if code >= 100 && code < 200 {
        HttpResponse::Informational(code)
    } else if code >= 200 && code < 300 {
        HttpResponse::Successful(code)
    } else if code  >= 300 && code < 400 {
        HttpResponse::Redirection(code)
    } else if code >= 400 && code < 500 {
        HttpResponse::BadRequest(code)
    } else if code >=500 && code < 600{
        HttpResponse::ServerError(code)
    }
    else {
        HttpResponse::Invalid(code)
    }
}
// task 7..end..