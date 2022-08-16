use core::slice;

fn main() {
    let mut num = 5;

    //Unsafe rust has two types of raw pointers that are similar to references
    let r1 = &num as *const i32; // immutable raw pointer, can't be direcely assigned after being deref
    let r2 = &mut num as *mut i32; // mutable raw pointer

    // Raw pointer bypass borrow rules, not guarenteed to point to valid mem, are allowed to be null, and don't implement automatic cleanup. 
    // Rust allows us to create raw pointers but we can't deref them if we are not in unsafe block.

    unsafe{
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // We created immut and mutable raw pointer pointing to same location in memory, but can't do that with reference since it will break ownership rules.
    // Raw pointers are still useful when interfacing with C code or building safe abstractions that borrow checker doesn't understand.

    // Unsafe methods and function

    unsafe fn dangerous (){} // unsafe means that when calling functions we need to put in the correct arguments, it implies we read the function's documentation and are taking responsibility for upholding the function contract.

    unsafe{ // If we remove the unsafe block it will call an error. The body of unsafe function is an unsafe block so we don't have to write an unsafe block inside function.
        dangerous();
    }

    // Safe abstraction of unsafe code 
    
    //Just because a function contains unsafe code doesn't make it an unsafe function. We can wrap unsafe code inside a safe function.
    
    //Example: split_at_mut(), is a safe method that split a slice into two slices along index passed in.

    let mut v = vec![1,2,3,4,5,6];

    let r = &mut v[..];

    let (a,b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);


    // functions that call external code 
    

    // sometimes we need to interact code in another language. In this case Rust has the extern keyword, which let's use a foreign language interface, ffi. 
    // A ffi is a way for a prog to define a function that another language can call.

    extern "C"{ //"C" defines which application binary interface, ABI the external function uses. ABI defines how to call the function at the assembly level.
    // abs function from C std library.
        fn abs(input: i32) -> i32; // Specify the name and signature of foreign function we want to call.
    }
    
    // calling function in extern block is always unsafe.
    unsafe{
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Allow other languages to call our Rust functions, by using extern keyword infront of function signature.

    #[no_mangle] // no_mangle is to let the compile know not mangle(when the compiler changes the name of our function) our function name
    pub extern  "C" fn call_from_c(){
        println!("Just called a Rust function from C!");
    }


    // Ability to access and modify mutable static variables.

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    
    unsafe{ // Can only access COUNTER in unsafe block.
        println!("COUNTER: {}", COUNTER);
    }


}



// Implementing split_at_mut using only safe code vs:

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // let len = slice.len();

    // assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..]) // Error: Cannot borrow slice as mutable more than once at a time. 
    // Borrow checker can't understand that we are borrowing different parts of our slice. We know our code is ok but Rust can't figure it out. 
    // We will use unsafe block: 


    // Recall slice are pointer to data and length of data.
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // Get raw mutable to our slice

    assert!(mid<=len); 

    unsafe{ 
        (
            slice::from_raw_parts_mut(ptr, mid), // Creates a new slice with pointer to some data and length
            slice::from_raw_parts_mut(
                ptr.add(mid), len-mid // ptr.add returns pointer after some offset.
            ),
        )
    }
    // From raw_parts_mut() and add() method are unsafe since they must trust that the raw pointer passed in and passed out are valid.  
} // Note even if function has unsafe block it is safe


// Global variables in rust are supported but can be problematic with ownership rules. In rust global vartiables are called static variables.

static  HELLO_WORLD: &str = "Hello, world!"; // Convention to write variable in screaming snakecase. We must also annotate the life time of variable and it must have a static lifetime. In this case don't have to specify it since Rust infers the lifetime of &str

// Differences with immutable static and constants. Static variables have fixed address in memory, constants can duplicate their data whenever they are used,
// If we are referencing constant in code base, the compiler can replace constant with concrete value.

//static variables can be mutable but accesing an modifying mutable static variables in unsafe.
static  mut COUNTER: u32 = 0;

fn add_to_count(inc:u32){
    unsafe{
        COUNTER += inc;
    }
}



    // Implementing an unsafe trait

    // A trait is unsafe if at least one its methods is unsafe
unsafe trait Foo{
    // methods go here
}

unsafe impl Foo for i32{
    // method implementation goes here.
}

// Fifth ability with unsafe code is able to access field of unions. A union is similar to a struct but only one field is used in each instance. Primarily used to interface with C unions, it is unsafe to access field of a union since Rust can't guarrentee what type stored in union is for a given instance.