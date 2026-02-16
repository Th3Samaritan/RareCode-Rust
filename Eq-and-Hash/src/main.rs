use std::collections::{HashSet, HashMap};

// task 2
#[derive(Hash, Eq, PartialEq)]
enum Letters {
    A,
    B,
}
// task 2..end..

// task 5 Type must be hashable to be used as a HashMap key
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Animal {
    Dog,
    Cat,
    Fish,
}
// task 5..end..

// task 6 Exercise: Shape array
#[derive(Eq, PartialEq, Hash)]
pub enum Shape {
    Circle,
    Triangle,
    Square,
}

#[derive(Debug)]
pub enum Comparison {
    Identical,
    SameGroup,
    NotEqual,
}

type ThreeShape = [Shape; 3]
// task 6..end..

// task 7 Exercise: Vote Counter
#[derive(Debug, Clone, Copy,PartialEq, Eq, Hash)] 
pub enum Vote {
    Yes,
    No,
    Abstain,
}
// task 7..end..

// task 8
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub fn deduplicate_priorities(priorities: &[Priority]) -> HashSet<Priority> {
    priorities.iter().copied().collect()
}
// task 8..end..

// task 9 Exercise: Remove below threshold
pub enum Letters {
    A,
    B,
    C,
    D
}
// task 9..end..

// task 10 Exercise: To Larger Enum
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Letter32 {
    A(i32),
    B(i32),
    C,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Letter64 {
    A(i64),
    B(i64),
    C,
}
// task 10..end..
fn main() {
    // task 1 Floats cannot be put into a set
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    
    println!("Float set: {:?}", set);
    // task 1..end..

    // task 2
     let _set = HashSet::from([Letters::A, Letters::B]);
     // task 2..end..

     // task 4 HashSets cannot be put in HashSets
      let s1 = HashSet::from([1, 2, 3]);
    let s2 = HashSet::from([4, 5, 6]);
    let s3 = HashSet::from([7]);
    let v1: Vec<i32> = s1.into_iter().collect();
    let v2: Vec<i32> = s2.into_iter().collect();
    let v3: Vec<i32> = s3.into_iter().collect();
     let final_set = HashSet::from([v1, v2, v3]);
    println!("{:?}", final_set);
    // task 4...end..

    // task 5 Type must be hashable to be used as a HashMap key
    let animals = [Animal::Dog, Animal::Cat, Animal::Cat, Animal::Dog, Animal::Fish, Animal::Dog];
    
    let result = count_animals(&animals);
    println!("{:?}", result);
    // task 5..end..

    // task 6
    let a: ThreeShape = [Shape::Circle, Shape::Triangle, Shape::Triangle];
    let b: ThreeShape = [Shape::Triangle, Shape::Triangle, Shape::Circle];
    
    let result = equality_test(a, b);
    println!("{:?}", result);
    // task 6..end..

    // task 7 Exercise: Vote Counter
    let votes = vec![
        Vote::Yes,
        Vote::No,
        Vote::Yes,
        Vote::Abstain,
        Vote::Yes,
    ];
    
    let results = count_votes(votes);
    println!("Vote counts: {:?}", results);
    // task 7..end..

    // task 8
    let tasks = vec![
        Priority::High,
        Priority::Low,
        Priority::High,
        Priority::Critical,
        Priority::Low,
    ];
    
    let unique_priorities = deduplicate_priorities(&tasks);
    println!("Original still exists: {:?}", tasks);
    println!("Unique priorities: {:?}", unique_priorities);
    // task 8..end..

    // task 9 Exercise: Remove below threshold
    let mut map = HashMap::from([(Letters::A, 10), (Letters::B, 9)]);
    let threshold = 10;

    remove_below_t(&mut map, threshold);
    println!("{:?}", map);
    // task 9..end..

    // task 10 Exercise: To Larger Enum
     let set = HashSet::from([Letter32::A(-16), Letter32::B(2), Letter32::C]);
    let result = upsize(&set);
    println!("{:?}", result);
    // task 10..end..
}

// task 5 Type must be hashable to be used as a HashMap key
pub fn count_animals(animals: &[Animal]) -> HashMap<Animal, usize> {
     let mut map = HashMap::new();
    for a in animals {
        if let Some(count) = map.get(a) {
            map.insert(*a, count + 1);
        } else {
            map.insert(*a, 1);
        }
    }
    map
}
// task 5..end..

// task 6 Exercise: Shape array
pub fn equality_test(a: ThreeShape, b: ThreeShape) -> Comparison {
     if a == b {
        return Comparison::Identical;
    }
    let set_a: HashSet<Shape> = a.into_iter().collect();
    let set_b: HashSet<Shape> = b.into_iter().collect();
    
    if set_a == set_b {
        return Comparison::SameGroup;
    }
    Comparison::NotEqual
}
// task 6..end..

// task 7 Exercise: Vote Counter
pub fn count_votes(votes: Vec<Vote>) -> HashMap<Vote, usize> {
    let mut counts = HashMap::new();
    
    for vote in votes {
    if let Some(count) = counts.get(&vote) {
    counts.insert(vote, *count + 1);
    } else {
    counts.insert(vote, 1);
        }
    }
    
    counts
}
// task 7..end..

// task 9 Exercise: Remove below threshold
pub fn remove_below_t(map: &mut HashMap<Letters, i32>, t: i32) {
    *map = map.iter().filter(|(_, &v)| v >= t).map(|(&k, &v)| (k, v)).collect();
}

// task 9..end..

// task 10 Exercise: To Larger Enum
pub fn upsize(set: &HashSet<Letter32>) -> HashSet<Letter64> {
    set.iter().map(|&l| {
        match l {
    Letter32::A(x) => Letter64::A(i64::from(x)),
    Letter32::B(x) => Letter64::B(i64::from(x)),
            Letter32::C => Letter64::C,
        }
    }).collect()
}
// task 10..end..