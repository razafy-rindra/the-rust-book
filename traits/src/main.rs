use std::iter::Sum;


pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

 impl Summary for NewsArticle{
    fn summarize_author(&self) -> String { 
        format!("{}", self.author) 
    }
    // Don't need to implement summarize since there is a default implementations
 }

pub struct Tweet{
    pub username: String,
    pub content: String, 
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for Tweet{
    fn summarize_author(&self) -> String {// Need to implement summarize author since no default implementation. 
        format!("@{}", self.username) 
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
pub trait Summary{ //Define shared methods for our types
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author()) //Default implementation
    }
}

 pub fn notify(item: &impl Summary){ // This function takes in a reference to something that implements summary
     println!("Breaking news! {}", item.summarize());
 }


//This above &impl syntax is syntax sugar for a "trait bound" which looks like this:

// pub fn notify<T:Summary>(item: &T){ // Generic that is limited to something that interprets a Summary trait.
//     println!("Breaking news! {}", item.summarize());
// }

// What if we want our function to take in multple inputs of the same type. Can't do that with the &impl syntax
// pub fn notify<T:Summary>(item1: &T, item2: &T){
//     //...
//}


// We can also specify multiple traits

// impl syntax:


// pub fn notify(item1: &(impl Summary + Display), item2: &impl: Summary){
//     ...
// }

// //Trait bound syntax

// pub fn notify<T:Summary+Display>(item1: &T, item2: &T){
//     ....
// }

// Specifying multiple trait bounds can hinder readablity, like for example:

// fn some_function<T:Display+Clone, U: Clone + Debug>(t:&T, u:&U) ->i32{....}


// To make it more readible we can use the "where"-clause

// fn some_function<T,U>(t:&T,u:&U) ->i32
//     where T:Display+Clone,
//           U: Clone+Debug
// {
//     ///..
// }


// Now let us talk about return types.

fn returns_summarizable() -> impl Summary  {//We return any type that implements the summary trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
} // This is very useful with closures and iterators which we will learn more about later.

// While this function can return any type that implements Summary it has it's limits. 

// fn return_bool(switch: bool) ->impl Summary{
//     if swtch{
//         NewsArticle{
//             ///
//         }
//     } else{
//         Tweet{
//             ///
//         }
//     }
// }

//This above code does not compile. We can only return one type.

//Trait bounds to conditionally implement methods

// struct Pair<T>{
//     x: T,
//     y: T,
// }

// impl<T> Pair<T>{ //This implementation block is for any pair struct.
//     fn new(x: T, y: T) -> Self{ // Every pair struct will have this method
//         Self {x,y}
//     }
// }

// impl<T: Display + PartialOrd> Pair<T>{ // We use trait bounds to say that T has to implement display and partial order
//     fn cmp_display(&self){ // only the struct that, where T implemement display and partial order will have this
//         if self.x >= self.y{
//             println!("The largest member is x = {}", self.x);
//         } else{
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }


// Blanket implementation

// We can implement a trait on a type that implements another strait


// impl<T:Display> ToString for T{ // We implement the ToString trait on any type that implement Display
//     //
// }

fn main(){
    let tweet = Tweet{
        username: String::from("@user"),
        content: String::from("Hello world"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle{
        author: String::from("The Author"),
        headline: String::from("I wrote an article!"),
        content: String::from("This is my article")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);

    println!("{}", returns_summarizable().summarize());
}