// procedural macros are defined using this similar syntax.

// use proc_macro; // Defines TokenStream

// #[some_attribute] // attribute specifies the kind of procedural macro we are creating. 
//pub fn some_name(input: TokenStream) -> TokenStream{ // Name of function is name of macro.
    //..
//}

// Token smallest elemetns of a program, for e.g. keywords, identifiers, operators, seperators, litterals.

pub trait HelloMacro{
    fn hello_macro();
}

// What if we wanted our default implementation of this function to print hello macro followed by the type on which the trait is implemented on. We must use our macro to generate the defaule implementation.




