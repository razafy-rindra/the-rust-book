use unicode_segmentation::UnicodeSegmentation; // Let's us iterate over grapheme clusters

fn main() {
    //Strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new(); //Like vectors we can create new
    let s2 = "initial contents";
    let s3 = s2.to_string(); // Create String from string slice with to_string
    let s4 = String::from("intial contents"); // Create String from string slice with from

    let mut s = String::from("foo");
    s.push_str("bar"); // Just like vector, string can grow and shrink in size. This method takes in a string slice.
    s.push('!'); // can also append characters

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s4 = format!("{}{}", s1,s2); // Can concatenate with format! which doesn't take ownershp of s1 or s2.
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s4);
    let s3: String = s1+&s2; // Can also append strings with the '+' operator. We are moving ownership from s1 to s3 and taking all the characters in s2 and appending them into s3.
    println!("Once again; {}", s3);
    // println!("{}", s1); Error, since we cannot borrow value after it has been moved.

    let hello: String = String::from("Hello");
    // let c: char = hello[0]; Error: string cannot be indexed by an integer. Each of characters can be from 1 to 4 bytes long, so we can't use the syntex hello[0]
    // To get the first character since that syntax gives us the first byte of the string, but that may not correspond with the length of the character.

    let hello: String = String::from("नमस्ते");

    //Bytes [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    for b in "नमस्ते".bytes(){
        println!("{}", b);
    }

    println!("-------");

    //Scalar values
    //['न', 'म', 'स', '्', 'त', 'े']

    for c in "नमस्ते".chars(){
        println!("{}", c);
    }

    //Grapheme clusters
    //["न", "म", "स्", "ते"], in order to keep the Rust standard libary lean, the abiliity to iterate over grapheme clusters in not included by default.

    println!("-------");

    for g in "नमस्ते".graphemes(true){
        println!("{}", g);
    }
}



