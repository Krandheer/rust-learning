use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let greeting_file_result = File::open("hello.txt");
    let mut greeting = match greeting_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut text = String::new();

    match greeting.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

fn read_file2() -> Result<String, io::Error> {
    let mut text = String::new();
    File::open("hello.txt")?.read_to_string(&mut text)?;
    Ok(text)
}

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file {:?}", e),
            },
            other_error => {
                panic!("problem opening the file {:?}", other_error);
            }
        },
    };
    println!("{:?}", greeting_file);
    let result = read_file();
    let result2 = read_file2();
    println!("{:?}", result);
    println!("{:?}", result2);
}
