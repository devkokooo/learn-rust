fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x: {x}, y: {y}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}, {six_point_four}, {one}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5]; // same value, 5 times

    let first = a[0];
    let second = a[1];
    println!("{first}, {second}");

    another_function(42);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    println!("fn five() returns: {}", five());

    let x = plus_one(5);
    println!("The value of x is: {x}");

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let temp_f = 32.0;
    println!("{temp_f}F to C is: {}", f_to_c(temp_f));
    let temp_c = 0.0;
    println!("{temp_c}C to F is: {}", c_to_f(temp_c));

    // 0, 1, 1, 2, 3, 5, 8, 13
    println!("fib(5) should be 5, actual: {}", fib(5));
    println!("fib(6) should be 8, actual: {}", fib(6));
}

fn another_function(x: i32) {
    println!("Another function");
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * 5.0/9.0
}

fn c_to_f(temp: f32) -> f32 {
    (temp * 9.0/5.0) + 32.0
}

// fib(5) = 5
// fib(6) = 8
fn fib(n: i32) -> i32 {
    let mut fib_n_minus_1 = 0;
    let mut fib_n = 1;

    let mut remaining = n;
    while remaining > 0 {
        let next = fib_n_minus_1 + fib_n;
        fib_n_minus_1 = fib_n;
        fib_n = next;

        remaining -= 1;
    }
    fib_n_minus_1
}
