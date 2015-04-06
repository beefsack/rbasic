extern crate std;

pub mod io;

use std::io::Read;
use self::io::PeekRead;

pub fn parse() {
    let input = "farts and cheese oh yes please";
    println!("Input is: {}", input);
    let mut reader = io::PeekReader::new(std::io::BufReader::new(input.as_bytes()));
    let mini: &mut[u8] = &mut[0; 5];
    let med: &mut[u8] = &mut[0; 10];

    println!("mini: {}", std::string::String::from_utf8_lossy(mini));
    println!("med: {}", std::string::String::from_utf8_lossy(med));

    println!("peeking into mini");
    assert!(reader.peek(mini).is_ok());
    println!("mini: {}", std::string::String::from_utf8_lossy(mini));

    println!("peeking into med");
    assert!(reader.peek(med).is_ok());
    println!("med: {}", std::string::String::from_utf8_lossy(med));

    println!("reading into mini");
    assert!(reader.read(mini).is_ok());
    println!("mini: {}", std::string::String::from_utf8_lossy(mini));

    println!("peeking into mini");
    assert!(reader.peek(mini).is_ok());
    println!("mini: {}", std::string::String::from_utf8_lossy(mini));

    println!("peeking into med");
    assert!(reader.peek(med).is_ok());
    println!("med: {}", std::string::String::from_utf8_lossy(med));
}
