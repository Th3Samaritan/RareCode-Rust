# Rust Learning Summary from Rarecode.ai

summary of some Rust basics, ownership, collections, iterators, traits, generics, and advanced fundamentals completed.

---

# 1. Rust Basics

## Variables & Mutability

```rust
let x = 5;
let mut y = 10;
y = 20;
```

Variables are immutable by default. Use `mut` to allow reassignment.

---

# 2. Control Flow

## if / else

Conditional branching.

```rust
let age = 18;
if age >= 18 { println!("Adult"); }
```

## if let

Cleaner pattern matching for one variant.

```rust
let opt = Some(5);
if let Some(v) = opt { println!("{}", v); }
```

## loop / while / for

```rust
loop { break; }

while false {}

for i in 0..5 { println!("{}", i); }
```

Use `for` for collections. Use `while` for conditions. Use `loop` for intentional infinite loops.

---

# 3. Integers & Floating Points

## Integers

Signed: i8, i16, i32, i64, i128, isize
Unsigned: u8, u16, u32, u64, u128, usize

## Floating Points

f32 and f64

```rust
let pi: f64 = 3.14;
let inf = f64::INFINITY;
let nan = f64::NAN;
```

Use f64 by default.

---

# 4. Ownership & References

* One owner per value
* `&T` immutable borrow
* `&mut T` mutable borrow
* Cannot borrow mutable and immutable simultaneously (E0502)
* Cannot have multiple mutable borrows (E0499)

```rust
fn print(s: &String) {}
```

---

# 5. Type Casting & Conversion

## as

```rust
let x = 5 as f64;
```

## from / into

```rust
let s = String::from("hello");
let s2: String = "world".into();
```

## try_from / try_into

```rust
use std::convert::TryInto;
let num: u8 = 5_i32.try_into().unwrap();
```

## Turbofish

```rust
let v = Vec::<i32>::new();
```

---

# 6. Vectors

Dynamic growable collection.

```rust
let mut v = vec![1,2,3];
v.push(4);
```

## iter, into_iter, iter_mut

```rust
v.iter();        // &T
v.into_iter();   // T (consumes)
v.iter_mut();    // &mut T
```

Use `.copied()` when iterating references of Copy types.

---

# 7. Sets

## HashSet

Unique values only.

```rust
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert(1);
```

Requires `Eq + Hash`.

---

# 8. HashMaps

Key-value storage.

```rust
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("a", 1);
```

HashMap II: entry API

```rust
map.entry("a").or_insert(0);
```

---

# 9. Arrays & Slices

```rust
let arr = [1,2,3];
let slice: &[i32] = &arr[0..2];
```

Fixed length. Slices borrow data.

---

# 10. Tuples

```rust
let t = ("Alice", 25);
let (name, age) = t;
```

Used to return multiple values.

---

# 11. Iterators

## Core Methods

```rust
let v = vec![1,2,3];
let sum: i32 = v.iter().sum();
```

### map

Transforms items.

```rust
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

### filter

Keeps matching items.

```rust
let even: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
```

### enumerate

```rust
for (i, val) in v.iter().enumerate() {}
```

### rev and step_by

```rust
for i in (0..10).rev().step_by(2) {}
```

---

# 12. Strings

## String vs &str

* `String` = owned
* `&str` = borrowed slice

```rust
let s = String::from("Hello");
let slice = &s;
```

Concatenation:

```rust
let full = s + " world";
```

Idiomatic to_string:

```rust
let num = 5.to_string();
```

---

# 13. Option

Represents presence or absence.

```rust
fn find() -> Option<i32> { Some(5) }
```

---

# 14. Result

Represents success or failure.

```rust
fn divide(a:i32,b:i32)->Result<i32,String>{
    if b==0 {Err("zero".into())} else {Ok(a/b)}
}
```

---

# 15. Enums

```rust
enum Status { Active, Inactive }
```

Enum as wrapper:

```rust
enum Id { User(u32), Admin(u32) }
```

---

# 16. Match

```rust
match 1 {
    1 => println!("One"),
    _ => println!("Other")
}
```

---

# 17. Structs

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}
```

---

# 18. impl

Attach methods to structs.

```rust
impl User {
    fn new(name:String, age:u32)->Self{
        Self{name,age}
    }
}
```

Associated functions & constants:

```rust
impl User {
    const MAX_AGE: u32 = 150;
}
```

---

# 19. Traits

Define shared behavior.

```rust
trait Speak { fn speak(&self); }
```

Trait bounds:

```rust
fn print<T: Speak>(item:T){}
```

Multiple constraints:

```rust
fn print<T: Speak + ToString>(item:T){}
```

Supertraits:

```rust
trait A {}
trait B: A {}
```

---

# 20. Generics

```rust
fn identity<T>(value:T)->T{ value }
```

---

# 21. PartialEq, Eq, PartialOrd, Ord

Enable comparisons.

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point { x:i32 }
```

---

# 22. Eq and Hash

Required for HashMap / HashSet keys.

```rust
use std::hash::Hash;
```

---

# 23. Dereferencing

```rust
let x = 5;
let r = &x;
println!("{}", *r);
```

Non-Copy types can be dereferenced if not moved.

---

# 24. Mutable References & Iterators

```rust
for v in vec.iter_mut() {
    *v += 1;
}
```

Cannot have two mutable borrows simultaneously.

---

# 25. Map & Filter on Tuples

```rust
let v = vec![(1,2),(3,4)];
let first: Vec<i32> = v.iter().map(|(a,_)| *a).collect();
```

---

# 26. Ranges

```rust
0..5
0..=5
```

---

# 27. From / Into / TryFrom / TryInto

Safe and idiomatic conversions between types.

---

# 28. Floating Point Edge Cases

* INFINITY
* NEG_INFINITY
* NAN

Use `is_nan()` to check.

---



---

# 29. Common Compiler Errors

Understanding these makes you fluent in Rust.

---

## E0502: Cannot borrow as mutable because it is also borrowed as immutable

Cause: Trying to mutably borrow while immutable borrow is active.

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s; // ❌ E0502
```

Fix: Ensure immutable borrow ends before mutable borrow.

```rust
let mut s = String::from("hello");
{
    let r1 = &s;
    println!("{}", r1);
}
let r2 = &mut s; // ✅
```

---

## E0499: Cannot borrow as mutable more than once at a time

Cause: Multiple mutable references simultaneously.

```rust
let mut x = 5;
let r1 = &mut x;
let r2 = &mut x; // ❌ E0499
```

Fix: Only one mutable reference at a time.

---

## E0382: Use of moved value

Cause: Ownership moved and then reused.

```rust
let s = String::from("hi");
let s2 = s;
println!("{}", s); // ❌ moved
```

Fix: Borrow or clone.

```rust
let s = String::from("hi");
let s2 = &s; // borrow
```

---

## E0277: Trait bound not satisfied

Cause: Type does not implement required trait.

```rust
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
struct User { id: u32 }
```

Fix: Derive required traits (`Hash`, `Eq`, `PartialEq`).

---

## E0308: Mismatched Types

Cause: Expected one type, found another.

```rust
let x: i32 = "hello"; // ❌
```

Fix: Ensure correct type or convert properly.

---

## E0599: No method found

Cause: Method requires trait in scope.

```rust
use std::convert::TryInto;
```

Bring required trait into scope.

---

# 31. Additional Core Concepts (Deep Coverage)

---

## References & Mutable References

```rust
let x = 5;
let r = &x;        // immutable reference
let mut y = 10;
let r2 = &mut y;   // mutable reference
```

Rules:

* Many immutable OR one mutable
* Not both at same time

---

## Dereferencing

```rust
let x = 5;
let r = &x;
println!("{}", *r);
```

Access value behind reference using `*`.

---

## clone() on a Reference

```rust
let s = String::from("hi");
let r = &s;
let cloned = r.clone();
```

Clones underlying value if type implements `Clone`.

---

## Copy Trait

Copy types duplicate automatically (stack types).

```rust
let x = 5;
let y = x; // no move
```

Common Copy types: integers, bool, char.

---

## Arrays

```rust
let arr = [1,2,3,4];
println!("{}", arr.len());
```

Fixed size, same type.

---

## chars()

Iterate characters in String.

```rust
let s = "abc";
for c in s.chars() {
    println!("{}", c);
}
```

---

## iter(), into_iter(), iter_mut()

```rust
let mut v = vec![1,2,3];

v.iter();        // &T
v.into_iter();   // T (consumes)
v.iter_mut();    // &mut T
```

---

## Mutable Iteration

```rust
let mut v = vec![1,2,3];
for n in v.iter_mut() {
    *n += 1;
}
```

---

## Mutable Parameters

```rust
fn add_one(x: &mut i32) {
    *x += 1;
}
```

---

## if let

Cleaner pattern match.

```rust
let opt = Some(10);
if let Some(v) = opt {
    println!("{}", v);
}
```

---

## try_into / try_from

```rust
use std::convert::TryInto;

let x: i32 = 10;
let y: u8 = x.try_into().unwrap();
```

Used when conversion may fail.

---

## to_string()

```rust
let n = 5;
let s = n.to_string();
```

Idiomatic string conversion.

---

## Iterator Utility Methods
**Hint: look at what functions are available to you in the docs**: https://doc.rust-lang.org/std/iter/trait.Iterator.html
```rust
let v = vec![1,2,3,4];

v.is_empty();
v.iter().count();
v.iter().min();
v.iter().max();
v.iter().sum::<i32>();
v.iter().nth(1);
v.iter().last();
let mut iter = v.iter();
iter.next();
```

---

## Extending Collections

```rust
let mut v1 = vec![1,2];
let v2 = vec![3,4];

v1.extend(v2);
```

---

## Enums (Deep)

```rust
enum Message {
    Text(String),
    Number(i32),
}
```

Enums can store data (wrapper pattern).

---

## Multiple Traits & Trait Bounds

```rust
fn print<T: Clone + ToString>(item: T) {}
```

---

## Supertraits & Subtraits

```rust
trait A {}
trait B: A {}
```

`B` requires `A`.

---

## Deriving Traits

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point { x: i32 }
```




