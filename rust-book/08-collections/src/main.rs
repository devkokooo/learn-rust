use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let _v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // might panic if index doesn't have element
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // safer way to deal with potential errors
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // ITERATING
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{i}");
        *i += 50
    }

    // ENUMS TO STORE MULTIPLE TYPES
    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(69.42),
    ];
    let cell = row.pop();
    println!("the cell is {cell:#?}");

    {
        let _v = vec![1, 2, 3, 4];
        // v in scope, can do stuff with it
    } // v out of scope, dropped and memory freed

    // STRINGS COLLECTIONS
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();

    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // this takes a string slice, avoids transferring ownership
    s1.push_str(s2);
    println!("s2 is {s2}, combined is {s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 transferred ownership
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // HASH MAPS
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score} points for team {team_name}!");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
