use std::sync::{Arc,Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter); // At each iteration counter gets moved, so counter needs many owners
        let handle = thread::spawn(move ||{ // Error: Since Rc pointer is not thread safe
            let mut num = counter.lock().unwrap();

            *num +=1;
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
