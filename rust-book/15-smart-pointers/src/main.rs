use std::cell::RefCell;
use std::{ops::Deref, rc::Rc};
use std::mem::drop;

use crate::List::{Cons, Nil, RCons, RefCons, RefCycle};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    RCons(i32, Rc<List>),
    RefCons(Rc<RefCell<i32>>, Rc<List>),
    RefCycle(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            RefCycle(_, item) => Some(item),
            _ => None,
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // BOX
    let b = Box::new(5);
    println!("b = {b}");

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // without deref coercion, we would have to type it like this!
    // hello(&(*m)[..]);
    hello(&m);

    // DROP
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");

    // RC - REFERENCE-COUNTED
    let a = Rc::new(RCons(5, Rc::new(RCons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = RCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // MULTIPLE OWNERS OF MUTABLE DATA
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RefCons(Rc::clone(&value), Rc::new(Nil)));

    let b = RefCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    // Reference Cycle
    let a = Rc::new(RefCycle(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(RefCycle(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // THIS WILL OVERFLOW THE STACK
    // println!("a next item = {:?}", a.tail());
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
