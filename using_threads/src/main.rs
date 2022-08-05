use std::thread;


fn main() {
    let v = vec![1,2,3];
    
    let handle = thread::spawn(move ||{ //closure takes ownership of values
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
