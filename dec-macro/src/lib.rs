// Implementation of vec! macro

#[macro_export] // This annotation means whenever the crate this macro is def in is brought in scope, this macro must be made available.
macro_rules! vec { // Note ! is not in not included in the name
 // Body of our macro similar to match expression, in this case with one arm.
    ($($x:expr),*) => { // Pattern we want to match on.
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }        
    };
}


// The pattern syntax in macros is different to match expression since we are matching against actual code.

// $() means capture any value that matches the pattern inside these paranthensis for use in replacement code.
// $x: expr, pattern matches any Rust expression and assignes it to the value $x.
// ,*: the comma means a litteral comma can appear in our code and * means our pattern can match zero or more times.


// The body of our arm is the code that will be executed. First we create a vector called temp_vec, then we have these three lines of code:

// $(
//     temp_vec.push($x); 
// )*

// This says generate temp_vec.push($x); for each match we get. So if our input is vec![1,2,3] the output will look like:

// temp_vec.push(1);
// temp_vec.push(2);
// temp_vec.push(3);


// Note Rust team is working on replacing macro_rules! with another kind of declarative macro.

