fn main() {
    // Mixing if let, else if, else if let, and else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite, color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    // for loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // function params
    let point = (3, 5);
    print_coordinates(&point);

    // matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    // matching multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values with ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring

    // structs
    let p = Point { x: 0, y: 7 };
    let Point { x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        _ => println!("don't worry about this")
    }

    // nested structs & enums
    let msg = Message::ColorChange(Color::Hsv(0, 160, 255));

    match msg {
        Message::ColorChange(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ColorChange(Color::Hsv(h, s, v)) => {
            println!("Change color the hue {h}, saturation {s}, and value {v}");
        }
        _ => println!("don't worry about this")
    }

    // structs and tuples
    let ((_feet, _inches), Point { x: _x, y: _y }) = ((3, 10), Point { x: 3, y: -10 });

    // ignoring values with _
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }
    foo(3, 4);

    // ignoring parts of a value with nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // unused variables
    let _x = 5;

    // _ does not bind to value
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");

    // remaining parts of a value with ..
    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // conditionals with match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ operator binding
    let msg = Msg::Hello { id: 5 };

    match msg {
        Msg::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Msg::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Msg::Hello { id } => println!("Found some other id: {id}")
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ColorChange(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Msg {
    Hello { id: i32 },
}
