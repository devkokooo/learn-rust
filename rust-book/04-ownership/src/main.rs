fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");

    let mut s = String::from("hello");
    change(&mut s);

    let my_string = String::from("hello world");
    // slices of String
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // references to String, which are whole slices of String
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";
    // slices of string literals, partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // string literats are already string slices
    let _word = first_word(my_string_literal);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // returned and gives to calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // comes into scope
    a_string // returns and moves back to calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}