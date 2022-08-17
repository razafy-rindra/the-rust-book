// Function Pointers. We can not only pass closures to functions, but functions into function with function pointer.

fn add_one(x: i32) -> i32{
    x+1
}

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32{
//     f(arg)+f(arg)
// }

// Unlike closures fn is a type rather than a trait, so we can specify it directly instead of using one of the function traits as a trait bound,

// Recall the three closure traits: Fn, FnMut and FnOnce (closure takes ownership). Function pointers implement all three closure traits, which is why it is best practice to use closures: 

fn do_twice<T>(f: T, arg: i32) -> i32
where T: Fn(i32)->i32{
    f(arg)+f(arg)
}


// One place where we only want to accept function pointers instead of function pointers and closures, is when interfacing with a language that dfoesn't use closures like C. 


fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);


    let list_of_numbers = vec![1,2,3];
    let list_of_strings: Vec<String> = 
        list_of_numbers
            .iter()
            // We pass in a closure in map function:
            //.map(|i| i.to_string())
            // We can also pass in a function pointer in map:
             .map(ToString::to_string)
            .collect();

    println!("{:?}", list_of_strings);


    // Other useful pattern with tuple.

    enum Status{
        Value(u32),
        Stop,
    }

    // Tuple string uses paranthesis to initialise values inside tuple like a function call. These initialiser are implemented as functions that take in args and returns tuple struct. 

    let list_of_statuses: Vec<Status> = 
        (0u32..20).map(Status::Value).collect(); // map treats Value like a function pointer.

    
}

// Returning closures from functions. We need to use impl trait syntax

fn returns_closure() -> impl Fn(i32) -> i32{
    |x| x+1
}

// This doesn't work in all cases for eg

// fn other_returns_closure(a: i32) -> impl Fn(i32) -> i32{
//     if a>0{
//         move |b| a+b
//     } else{
//         move |b| a-b // Error: no two closures even if identical have the same type. Consider Boxing our closure.
//     }
// }

// The two closures in our function have different types, and the impl trait synyax only works if we are returning one type. Instead of using the impl trait syntax we can return a trait object. 



fn other_returns_closure(a: i32) -> Box<dyn Fn(i32) -> i32>{ // We need to wrap our closure in Box smart pointer, since rust does not know the size of our trait object.
    if a>0{
        Box::new(move |b| a+b)
    } else{
        Box::new(move |b| a-b) 
    }
}