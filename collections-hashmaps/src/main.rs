use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10); //Note we are not passing the strings as references, so we transfer ownership into the hashmap
    scores.insert(yellow,50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //Get method takes a reference to a key and gives us an option, since we can't guarentee a value will be returned, if we pass an invalide key it will return None.

    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }

    println!("-------");

// Updating our Hashmap

    scores.insert(String::from("Blue"), 20); // Overwrites the Blue key with the value 20.

    //If we don't run to overwrite the existing values:

    scores.entry(String::from("Green")).or_insert(30); //If there isn't entry for key insert key with value 30 unless do nothing.
    scores.entry(String::from("Green")).or_insert(40); //Nothing happens here.

    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }
    
    println!("---");

// Updating values based on old value

    let text = "hello world wonderful world";
    // value is how many times the key appears in the string.
    let mut map = HashMap::new();


    //["hello", "world", "wonderful", "word"]
    for word in text.split_whitespace() { //Splits up the string by the whitespace
        let count = map.entry(word).or_insert(0); // .entry retirns an enum representing the value at that key. If the word doesn't exists it adds it to the hashmap and initialises the value to 0. If it does exists it doesn't do anything.
        *count += 1; //or_insert returns a mutable reference to our value, so we can deincrement it and add 1, even if it doesn't do anything sine the key already exists.
    }

    println!("{:?}", map);

}
