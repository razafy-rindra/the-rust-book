extern crate proc_macro; // Allows us to read and manipulate rust code

use proc_macro::TokenStream;
use syn; // Allows us to take string of rust code and turn into syntax tree data struct
use quote::quote; // Can take syntax tree data structure into rust code.

#[proc_macro_derive(HelloMacro)] // Indicates this is custom derived macro
pub fn hello_macro_derive(input: TokenStream) -> TokenStream{
    // Construct a representation of Rust code as a syntax tree we can manipulate
    let ast = syn::parse(input).unwrap(); // Appropriate to panic if error since this function is only called at compile time.

    // Build the trait implementation
    impl_hello_macro(&ast)
}


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream{
    let name = &ast.ident; // Extract out name of type we are working on.
    let gen = quote!{ // quote macro to output some rust code
        impl HelloMacro for #name{
            fn hello_macro(){
                println!(
                    "Hello, Macro! My name is {}!",
                    stringify!(#name) // takes an expression and turn it into string without evaluating it.
                );
            }
        }
    };
    gen.into()
}