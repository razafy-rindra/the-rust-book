use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]

struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,//Can't use Rc since it will cause a reference cycle.
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf), // 0
    );
    
    {
        let branch = Rc::new(Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // downgrade turns Rc smart pointer into weak smart pointer. 

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch), // 1, since leaf has a weak reference to branch
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2, since leaf is a child of branch
            Rc::weak_count(&leaf), // 0
        );

    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); //None since branch is dropped 
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf), // 0
    );
}
