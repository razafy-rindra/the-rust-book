// Procedural Macros are like functions, they take code as input operate on it and produce code as output. 
// In contrast with declarative macros which match against patterns and replace code with other code.

// Kinds of procedural macros:Custom derived, attribute-like, function-like

// For now procedural marcros must be defined in their own crate with a custom crate type.

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Two use statements since we have two crates hello_macro and hello_macro_derive. Each crate stil has to be published seperately and code using our crate has to bring each crate into scope.


#[derive(HelloMacro)]
struct Pancakes;

fn main(){
    Pancakes::hello_macro();
}


// Attribute-like macros, similar to custom derived macros expect instead of generating code for derive attribute we can create a custom attribute. This only works on struct and enums.

// #route[(GET, "/")]
// fn index(){
//     //..
// }

// #[proc_macro_attribute]
// pub fn route(
//     attr: TokenStream, // Get, "/"
//     item: TokenStream // fn index() {}
// ) -> TokenStream{
//     //..
// }

// Function-like macros look like function calls but more flexible. They can take a variable number of args and the operate on rust code.

// let sql = sql!(SELECT * FROM post WHERE id=1);

// #[proc_macro]
// puu fn sql(input: TokenStream) -> TokenStream{

// }