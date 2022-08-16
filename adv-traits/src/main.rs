// Associated types, placeholders added to traits and methods can use

pub trait Iterator{
    type Item; // associated type

    fn next(&mut self) -> Option<Self::Item>;
}

//What is the difference between an associated type and generic? They both allow us to define a type without specifying concrete value. In associated type only one concrete type per implementation, but generics can have many concrete types

struct Counter{}

impl Iterator for Counter{
    type Item = u32; // Can't have another implemtentation where item is different 

    fn next(&mut self) -> Option<Self::Item>{
        Some(0)
    }
}

// We can't have a second implementation like this:

// impl Iterator for Counter{
//     type Item = u16; 

//     fn next(&mut self) -> Option<Self::Item>{
//         Some(0)
//     }
// }


// But we can do this with generic.
pub trait GenericIterator<T>{
    fn next(&mut self) -> Option<T>;
}

// Two implementation of GenericIterator.

impl GenericIterator<u32> for Counter{ 
    fn next(&mut self) -> Option<u32>{
        Some(0)
    }
}

impl GenericIterator<u16> for Counter{ 
     fn next(&mut self) -> Option<u16>{
         Some(0)
     }
 }

 // When deciding between generics and iterators we need to know if we need multiple implementation for a single type. In the case of Iterators it makes sense to use associated type, since for any implementation we want next() to return the same type. 


 // Default Generic Type parameters and type overloading. 

 // Generic type parameters can specify a default concrete type so that implementers only need to specify a concrete type if it is not the default.

 // For example customizing the behaviour of op, aka. operator overlead. Rust can let us customise the semantics of certain op with associated traits in std lib, for example Add operator.

 use std::ops::Add;

 #[derive(Debug,PartialEq)]
 struct Point{
    x:i32,
    y:i32,
 }

impl Add for Point{
    type Output = Point;
    
    fn add(self,other:Point) ->Point{
        Point { 
            x: self.x+other.x, 
            y: self.x+other.x 
        } 
    }
}

// Implementation of Add trait

trait OurAdd<Rhs=Self>{ // Add trait has generic called Rhs(right hand side), it's default concrete type is self, the object implenting the add trait. So we don't have to specify it in our Points example 
    type Output;

    fn add(self,rhs:Rhs) -> Self::Output;
}


// Specifying generic

struct Millimeters(u32);
struct Meters(u32);

impl OurAdd<Meters> for Millimeters{
    type Output = Millimeters;

    fn add(self,rhs:Meters) -> Millimeters {
        Millimeters(self.0+(rhs.0*1000))
    }
}

// In general we can use default generic type param for two reasons:
    // Extend a type without breaking a code.
    // Adding customization for code that most users won't need.


//Calling method with the same name, we can have two traits with same methods and implement both those traits on one type. We can also implement on a type itself with the same name as in another method.

// In this case we need to tell Rust which method we want to call.

trait Pilot{
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

struct Human;

impl Human{
    fn fly(&self){
        println!("*waiving arms furiously*");
    }
}

impl Pilot for Human{
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human{
    fn fly(&self){
        println!("Up!");
    }
}
    // Since fly method takes self as a parameter, if we had two structs implemetning Wizard trait we know which method to call. This is not true for asssociated function.


    trait Pilot2{
        fn fly();
    }
    
    trait Wizard2{
        fn fly();
    }
    
    struct Human2;

    
    impl Human2{
        fn fly(){
            println!("*waiving arms furiously*");
        }
    }
    
    impl Pilot2 for Human2{
        fn fly() {
            println!("This is your captain speaking");
        }
    }
    
    impl Wizard2 for Human2{
        fn fly(){
            println!("Up!");
        }
    }

// Supertraits

// Trait dependent on functionality on other trait, the trait we rely on is called supertrait

// trait OutlinePrint {
//     fn outline_print(&self){
//         let output = self.to_string(); // Error calling to_string since don't know that self implements to_string, so we need to make sure that self does. to_string() is defined in Display trait so we want to make sure anything that implements OutlinePrint implemetents Display.
//         let len = output.len();
//         println!("{}", "*".repeat(len+4));
//         println!("*{}*", " ".repeat(len+2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len+2));
//         println!("{}", "*".repeat(len+4));
//     }
// }

use std::fmt; // This is where Display is defined.


trait OutlinePrint: fmt::Display { // This says that our trait depends on Display trait.
    fn outline_print(&self){
        let output = self.to_string(); 
        let len = output.len();
        println!("{}", "*".repeat(len+4));
        println!("*{}*", " ".repeat(len+2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len+2));
        println!("{}", "*".repeat(len+4));
    }
}


// What if we don't implement display triat? We get an error.


impl OutlinePrint for Point{}

impl fmt::Display for Point{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}


// New type pattern

// We learned about Orphan rule, we can define trait on a type as long as triat or type is defined in our crate, the newtype pattern allows us to get around this. We don this by create a tuple with one field which is the type we are wrapping.

// For example we want to implement Display trait for Vector which are both defined outside of our crate.

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// The problem is that the Wrapper type doesn't implement the methods implemented for vector. If we did want the new type to implement every mothod on type it is holding we can implement deref trait so that deref wrapper returns the value. If we don't want the wrapper to have all the methods we need to imnplemtent each method individually.

fn main() {
    assert_eq!(
        Point{x:1,y:0}+Point {x:2,y:3},
        Point{x:3,y:3}
    );

    let person = Human;
    person.fly(); // When we run we get *waiving arms furiously* the method on Human struct

    Pilot::fly(&person); // To run the Pilot implementation
    Wizard::fly(&person);// To run the Wizard implementation

    Human2::fly(); // Associated function in struct is called

    <Human2 as Wizard2>::fly(); // Fully qualified syntax to call fly function on Wizard2 trait as implemented for Human2 

    let p = Point{x:2,y:57};

    p.outline_print();

    let v = Wrapper(vec![String::from("1"),String::from("2"),String::from("Hello")]);

    println!("{}", v);

}

