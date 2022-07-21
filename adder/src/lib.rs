pub fn add_two(a:i32)->i32{
    internal_adder(a,2)
}

fn internal_adder(a:i32, b:i32) ->i32{
    a+b
}

// Convention that in the same file as our product code we have a module named test, that runs tests.
#[cfg(test)] // This makes it so that we only compile the bellow code when we run test.
mod tests { //Test module, holds our test.
    use super::*;

    #[test]
    fn internal(){
        assert_eq!(4, internal_adder(2, 2)); // internal_adder is private but we can call it since in rust, child modules can access anything in parent module including private fields.
    }
}
