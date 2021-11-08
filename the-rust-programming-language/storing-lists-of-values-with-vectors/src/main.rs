fn main() {
    //
    // vector store values of the same type
    //
    
    // creating an empty vector
    let mut v1: Vec<i32> = Vec::new();
    // creating a vector with elements
    let v2 = vec![1, 2, 3, 4, 5];

    // adding elements
    v1.push(4);
    v1.push(5);
    v1.push(6);
    v1.push(7);
    
    // reading elements
    let third_of_v1: &i32 = &v1[2];
    println!("The third element of v1 is {}", third_of_v1);

    // or
    match v2.get(2) {
        Some(third) => println!("The third element of v2 is {}", third),
        None => println!("There is no third element."),
    }

    // iterating vector
    for i in &v2 {
        println!("{}", i);
    }
    
    // iterating & modify vector
    for i in &mut v1 {
        // * -> dereference operator 
        *i += 50;
        println!("{}", i)
    }

    //
    // using an enum to store multiple types
    //
    #[derive(Debug)]
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

    for i in &row {
        println!("{:?}", i);
    }
}
