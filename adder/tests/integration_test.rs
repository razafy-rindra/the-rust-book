use adder;

mod common; //Module declaration it will look content of module in either a file called common.rs or a folder called common with a file called mod.rs 

#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, adder::add_two(2)); // Need to call the public API, can't call the inner adder function
}