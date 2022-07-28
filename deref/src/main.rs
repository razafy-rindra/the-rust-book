use std::ops::Deref; // Bring Deref trait into scope

struct MyBox<T>(T);

impl <T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x) // Note unlike MyBox, Box doesn't store data on the heap.
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T; // We will talk about this later

    fn deref(&self) -> &T{
        &self.0 //Return a reference to item stored in tuple struct.
    }
}

fn main (){
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str){
    println!("Hello, {}!", name);
}






