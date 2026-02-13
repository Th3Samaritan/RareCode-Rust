// task 1 Introduction to Enums
enum Color {
    White,
    Black,
}
// task 3
pub enum Color {
    Black,
    White,
}
// task 3..end..

// task 4 Matching more than just one enum
pub enum PrimaryColor{
    Red,
    Green,
    Blue,
}

pub enum MixedColor {
    Cyan,
    Magenta,
    Yellow,
    Red,
    Green,
    Blue,
}
// task 4..end..

// task 5 Exercise: Rock Paper Scissors
pub enum Outcomes {
    Player1,
    Player2,
    Tie
}

pub enum Items {
    Rock,
    Paper,
    Scissors
}
// task 5..end..

// task 10 Exercise: SchrodingersCat 1
pub enum SchrodingersCat{
    Alive,
    Dead, 
    Superposition,
}
// task 10..end..
fn main() {
    let color = Color::Black;

    match color {
        Color::White => println!("The color is White"),
        Color::Black => println!("The color is Black"),
    };
    // task 1..end..

    // task 2 Testing the value of an enum
    enum Color {
        Red,
        Green,
        Blue,
    }
    
    let c = Color::Green;
    
    let str_value = match c {
        Color::Red => "red",
        Color::Green => "green",
        Color::Blue => "blue",
    };
    
    println!("{}", str_value);
    // task 2..end..

    // task 3
    let c = Color::Black;
    
    match invert_color(c) {
        Color::White => println!("white"),
        Color::Black => println!("black"),
    };
    
    let c = Color::White;
    
    match invert_color(c) {
        Color::White => println!("white"),
        Color::Black => println!("black"),
    };
    // task 3..end..

    // task 4 Matching more than just one enum
     let color_1 = PrimaryColor::Red;
    let color_2 = PrimaryColor::Green;

    
    let result = mix_colors(color_1, color_2);
    let mix = match result {
        MixedColor::Magenta => "magenta",
        MixedColor::Yellow => "yellow",
        MixedColor::Cyan => "cyan",
        MixedColor::Red => "red",
        MixedColor::Green => "green",
        MixedColor::Blue => "blue",
    };
    
    println!("{}", mix);
    // task 4..end..

    // task 5 Exercise: Rock Paper Scissors
     let player_1_choice = Items::Rock;
    let player_2_choice = Items::Paper;
    
    let result = play_game(player_1_choice, player_2_choice);
    match result {
        Outcomes::Player1 => println!("player 1 wins"),
        Outcomes::Player2 => println!("player 2 wins"),
        Outcomes::Tie => println!("tie"),
    };
    // task 5..end..

    // task 6
     let color = get_color(0);
    match color {
        Some(Color::Black) => println!("black"),
        Some(Color::White) => println!("white"),
        None => println!("Invalid color"),
    }
    // task 6..end..

    // task 7 
      let color = Color::White;
    let result = from_color(color);
    println!("{}", result);
    // task 7..end..

    // task 8 Exercise: Vector of Enums to Vector of String
     let v: Vec<Color> = vec![Color::White, Color::Black];
    let result = translate_colors(v);
    println!("{:?}", result);
    // task 8..end..

    // task 9 Mutable References to Enums
     let mut color = Color::White;
    flip_color(&mut color);
    match color {
        Color::Black => println!("black"),
        Color::White => println!("white"),
    }
    // task 9..end..

    // task 10 Exercise: SchrodingersCat 1
    let mut cat = SchrodingersCat::Alive;

    put_in_box(&mut cat);
    println!("cat state: {}", cat_state(&cat));
    // task 10..end..
}

// task 3
pub fn invert_color(c: Color) -> Color {
    match c {
        Color::White => Color::Black,
        Color::Black => Color::White,
    } // no semicolon, we return the value
}
// task 3..end..

// task 4 Matching more than just one enum

pub fn mix_colors(c1: PrimaryColor, c2: PrimaryColor) ->  MixedColor{
    match (c1, c2) {
        (PrimaryColor::Red, PrimaryColor::Blue) => MixedColor::Magenta,
        (PrimaryColor::Blue, PrimaryColor::Red) => MixedColor::Magenta,
        (PrimaryColor::Green, PrimaryColor::Red) => MixedColor::Yellow,
        (PrimaryColor::Blue, PrimaryColor::Green) => MixedColor::Cyan,
        (PrimaryColor::Green, PrimaryColor::Blue) => MixedColor::Cyan,
        (PrimaryColor::Red, PrimaryColor::Red) => MixedColor::Red,
        (PrimaryColor::Green, PrimaryColor::Green) => MixedColor::Green,
        (PrimaryColor::Blue, PrimaryColor::Blue) => MixedColor::Blue,
        (PrimaryColor::Red,
            PrimaryColor::Green) =>
            MixedColor::Yellow,
    }
}
// task 4..end..

// task 5 Exercise: Rock Paper Scissors
pub fn play_game(p1: Items, p2: Items ) -> Outcomes {
    match (p1, p2) {
        (Items::Rock,Items::Rock) => Outcomes::Tie,
        (Items::Rock, Items::Scissors) => Outcomes::Player1,
        (Items::Rock, Items::Paper) =>
        Outcomes::Player2,
        (Items::Scissors, Items::Scissors) => Outcomes::Tie,
        (Items::Scissors, Items::Rock) =>
        Outcomes::Player2,
        (Items::Scissors, Items::Paper) =>
        Outcomes::Player1,
        (Items::Paper, Items::Paper) =>
        Outcomes::Tie, 
        (Items::Paper, Items::Rock) =>
        Outcomes::Player1,
        (Items::Paper, Items::Scissors) =>
        Outcomes::Player2,

    }
}
// task 5..end..

// task 6
pub fn get_color(i: u8) -> Option<Color> {
    match i {
    0=> Some(Color::Black),
    1 => Some(Color::White),
    _ => None,
    }
}
// task 6..end..

// task 7
pub fn from_color(color: Color) -> u8 {
    match color{
        Color::Black => 0,
        Color::White => 1,
    }
}
// task 7..end..

// task 8 Exercise: Vector of Enums to Vector of String
pub fn translate_colors(colors: Vec<Color>) -> Vec<String> {
    colors.into_iter().map(|color| match color {
            Color::White => "white".into(),
            Color::Black => "black".into(),
        }).collect()
}
// task 8..end..

// task 10 Exercise: SchrodingersCat 1
pub fn put_in_box(cat: &mut SchrodingersCat) {
    *cat = SchrodingersCat::Superposition;
}

pub fn cat_state(cat: &SchrodingersCat) -> String {
    match *cat {
        SchrodingersCat::Alive => "alive".into(),
        SchrodingersCat::Dead => "dead".into(),
        SchrodingersCat::Superposition => "superposition".into(),
    }
}

// task 10..end..

// task 11 Exercise: SchrodingersCat 2
pub fn open_box(cat: &mut SchrodingersCat, outcome:bool) {
    if outcome {
        *cat = SchrodingersCat::Alive;
    } else {
        *cat = SchrodingersCat::Dead;
    }
}
// task 11..end..