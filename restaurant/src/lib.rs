// In src/lib.rs
mod front_of_house; // This tells Rust, 
//define our module here but get the contents from a different file with the same name as our module.


pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
