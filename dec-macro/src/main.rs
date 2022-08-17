// Declarative Macros

// Macros allow us write code that writes other code. This is metaprogramming. We can think of Macros as functions whose input and output is code.

// Examples of macros we used are prinln! and vec!

// Key difference between macros and functions:
// Functions must declare the number of param they accept but macros can accept a variable number of param
// Functions are called at run time, macros are expanded before function finished compiler. 

// Macros are stronger than functions but put a level of complexity in our code.

// Two types of Macros: Declarative Macros and Procedural Macros.


fn main() {
// Dec Macros are most used form of macros in Rust, they let us write something similar to a match expression. 

// vec! declaritive macro, note we can pass in different types and a different amount of arguments in the macros:
    let v1: Vec<u32> = vec![1,2,3]; // 3 args
    let v2: Vec<&str> = vec!["a","b","c","d","e"]; // 5 args
}
