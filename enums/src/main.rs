//Using if let syntax

fn main(){
    let some_value = Some(3);
// Instead of using the match like this, when we only have on case we care about.    
    match some_value{
        Some(3) => println!("three"),
        _=> (),
    }

// We can use the if-let synyax

    if let Some(3) = some_value{ // Says if some_value matches with Some(3) execute the bellow code
        println!("three");
    }
}
