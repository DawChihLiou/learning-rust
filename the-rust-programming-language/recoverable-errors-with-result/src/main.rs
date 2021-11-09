use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },    
    };
    // Too many `match` expressions. `match` is very usefule but it's 
    // very much a primitive. We can rewrite the code above in closure:
    let f_closure = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let username = read_username_from_file()
        .expect("Failed reading username");
    println!("username is {}", username);

    let concise_username = concise_read_username_from_file()
        .expect("Failed to read concise username");
    println!("concise username is {}", concise_username);
}

//
// Propagating Errors
// The function doesn't handle errors. It returns the errors 
// to the calling function.
//
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Propagating errors is a very common pattern in Rust.
// We can use `?` operator to to make the code more concise.
fn concise_read_username_from_file() -> Result<String, io::Error> {
    // if the `Result` is an `Ok`, the line will return the value 
    // inside `Ok`.
    // if the `result` is an `Err`, the function will return the `Err`.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // `?` operator calls the `from` function, defined in the `From` trait.
    // `from` function convert the recieved error type into the error 
    // return type of the function.
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn more_concise_read_user_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // We can chain methods with `?` 
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Reading file into a string is a common operation. It's built in
// in `std::fs::read_to_string`.
//
use std::fs;
use std::io;

fun even_more_concise_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/*
 * We can only use `?` operator in a function that has the return 
 * type of `Result`, `Option`, or a type that implements 
 * `std::ops::Try`.
 *
 * Using `?` in `main()` is allowed but we need to change the return type:
 *
 * ```
 *  fn main() -> Result<(), Box<dyn Error>> {
 *      let f = File::open("hello.txt")?;
 *      Ok(())
 *  }
 * ```
 *
 * `Box<dyn Error>` type is a trait object. It means "any kind of error".
 *
 * `main()` function is a special function. There are restrictions 
 * on its return type but `Result<(), Box<dyn Error>>` is a valid 
 * return type of `main()` function. 
 */
