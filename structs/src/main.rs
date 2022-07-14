#[derive(Debug)] // Needed to print out, we will look more into this later.
struct Rectangle{
    width: u32,
    height: u32
}

// Creating implementation block for Rectangle struct,
// This will house function and methodes associated with struct

impl Rectangle{
    fn area(&self) -> u32{ // First arguement in method is always self, the instance the method is being called on
            self.width*self.height
    }

    fn can_hold(&self, other: &Rectangle) ->bool{
        self.width > other.width && self.height>other.height
    }
}

//Struct allow us to have multiple impl blocs

// We can also def associated function, unlike methods they aren't tied to an instance of our struct.

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle { 
            width: size, 
            height: size 
        }
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50
    };

    let rect1 = Rectangle{
        width: 20,
        height: 40
    };

    let rect2 = Rectangle{
        width: 40,
        height: 50
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("{:#?}", rect);
    println!("Square rectangle: {:#?}", rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

