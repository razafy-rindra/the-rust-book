use std::fmt;
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}


fn main() {
    let w = Wrapper(
        vec![String::from("Hello"), String::from("world")]
    );

    println!("w = {}", w);

    type Kilometers = i32; // Not a new type but a synonym for i32

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x+y = {}", x+y); // Kilometer treated like i32
}
