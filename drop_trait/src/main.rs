struct CustomSmartPointer{
    data:String,
}

impl Drop for CustomSmartPointer{ // Drop trait requires that we implement the drop method.
    fn drop(&mut self) {
        println!("Dropping CustomeSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("Dropped before end of main"); 
}
