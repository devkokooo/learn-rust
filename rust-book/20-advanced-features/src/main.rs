fn main() {
    // UNSAFE RUST

    // dereferencing raw pointer
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    // raw pointer to arbitrary memory address
    let address = 0x012345usize;
    let _r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // calling unsafe function or method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // creating safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // using extern functions to call external code
    println!("Absolute value of -3 according to C: {}", abs(-3));

    // accessing or modifying a mutable static variable
    println!("value is: {HELLO_WORLD}");

    unsafe {
        // SAFETY: this is only called from a single thread in `main`
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    // ADVANCED TRAITS

    // using default generic params and operator overloading
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let p = Point { x: 1, y: 0 };
    let sum = p.add(Point { x: 2, y: 3 });
    assert_eq!(sum, Point { x: 3, y: 3 });

    // disambiguating beetween identically named methods
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // implementing external traits with the Newtype pattern
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    // ADVANCED TYPES

    // type synonyms & type aliases
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // loooooooooooong types = need aliases
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Thunk = Box::new(|| println!("hi"));

    fn _takes_long_type(f: Thunk) {}
    fn _returns_long_type() -> Thunk {
        let f: Thunk = Box::new(|| println!("hi"));
        f
    }

    // dynamically sized types and the Sized trait
    let _s1: &str = "Hello there!";
    let _s2: &str = "How's it going?";

    // ?Sized means T may or may not be Sized
    // switching from fn generic<T:Sized>(t: T)
    // to the one below means we need to use some kind of pointer
    // because the type might not be Sized, so &T reference is used
    fn _generic<T: ?Sized>(t: &T) {}

    // ADVANCED FUNCTIONS AND CLOSURES
    let ans = do_twice(add_one, 5);
    println!("The answer is: {ans}");

    let list_of_numbers = vec![1, 2, 3, 4, 5];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("list_of_strings: {list_of_strings:?}");

    let list_of_numbers = vec![1, 2, 3, 4, 5];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings: {list_of_strings:?}");

    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // returning closures
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }

    // MACROS
    use advanced_features::myvec;
    let v: Vec<u32> = myvec![1, 2, 3];

    for val in v {
        println!("val {val} in myvec");
    }

    // proc macro
    use hello_macro::HelloMacro;

    #[derive(hello_macro_derive::HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();

    // attribute-like macros
    #[route(GET, "/")]
    fn _index() {
        println!("GET /");
    }

    // function-like macros
    let _sql = sql!("SELECT * FROM posts where id=1");
}

// creating safe abstraction over unsafe code
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

// using extern functions to call external code
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

// calling Rust functions from other languages
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// accessing or modifying a mutable static variable
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

/// SAFETY: calling this from more than a single thread at a time
/// is undefined behavior, so you *must* guarantee you only call it
/// from a single thread at a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// implementing unsafe traits
unsafe trait Foo {

}

unsafe impl Foo for i32 {

}

// defining traits with associated types
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// using default generic params and operator overloading
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// disambiguating beetween identically named methods
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Supertraits
use std::fmt;

use hello_macro_derive::{route, sql};

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// implementing external traits with the Newtype pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

// returning closures
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
