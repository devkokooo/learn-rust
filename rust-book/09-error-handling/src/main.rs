use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    let username_file = match read_username_from_file() {
        Ok(file) => file,
        Err(e) => panic!("Problem reading username from file: {e:?}"),
    };
    println!("{username_file}");

    match read_username_from_file_short() {
        Ok(uname) => println!("Found {uname} in file"),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                panic!("cannot find file to open");
            }
        }
    };

    match read_username_from_file_shorter() {
        Ok(un) => {
            let mut msg = String::from("We found ");

            for line in un.lines() {
                println!("{line}");
                msg.push_str(&(line.to_owned() + ", "));
            }
            println!("{msg}");
        },
        Err(e) => panic!("{}", e),
    }


}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
