use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str, //Need to specify lifetime since more than 1 input.
    ann: T, //Generic
) ->&'a str
where
    T:Display, //Triat bound limit T to only those that implement Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len(){
        x
    }else {
        y
    }
}

fn main(){
    let announcement = String::from("Grand finale");
    let x = "we did generics, traits and lifetimes";
    let y = "how fun!!!!";
    let result = longest_with_an_announcement(x, y, announcement);
    println!("These past sections {}, {}", result, y);
}