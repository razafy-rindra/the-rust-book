#![allow(unused)]
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>>{ //In the error case we return a "Trait" object, which we will talk about in chap 17 
    let f = File::open("hello.txt")?;

    Ok(())

}