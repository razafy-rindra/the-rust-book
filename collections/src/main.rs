fn main() {
//Vectors
    let a = [1,2,3];
    let mut v:Vec<i32> = Vec::new(); //Vectors can hold any values, so we need to specify the type.
    v.push(1); // Vectors can grow in size, we can add eleements to it with .push()
    v.push(2);
    v.push(3);

    {
        let v2 = vec![1,2,3]; // Can create a vector with initial values like this.
    }// When our scope ends, v2 and all elements in it are dropped.

//Acessing elemetents in vectors

    let mut v = vec![1,2,3,4,5];

    let third = &v[2];
    //  v.push(6);  Error since v is borred as immutable on previous line. If we have an immutable reference to something we expect the underlying value to not change. But if we take a mutable reference to the same thing, the underlying value could change.
    // When we push an element to a vector, we may need to allocated more memory for the new element, so we would move all elements in vector to new location, and our element at the previous line will be pointing to something else.
    println!("The third element is {}", third);

    // If we try to run with third = &v[20], we will get a runtime error, unlike with arrays, we don't get a compile time error. Since at compile time we don't know the size of vector. 

    //If we want to access data gracefully, so that the program doesn't crash at runtime if an invalid index is used.

    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

//Iterating over vector.

    for i in &mut v{ // for loop
        *i += 50; // Derefencing operator
        println!("{}", i);
    }

// Storing enum variables

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _=> println!("Not an integer!")
    };
}

