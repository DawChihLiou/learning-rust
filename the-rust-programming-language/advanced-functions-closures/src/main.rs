//
// `fn` type is called "function pointer". Use it to pass regular functions to functions.
//
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

//
// Returning closures
//
// We can't return closures directly because they are represented by traits.
// We can't return function pointer `fn` because it's not allowed by the compiler.
//
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // passing closure or function
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings_closure: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    // passing `to_string` function defined in `ToString` trait directly in map
    let list_of_strings_fn: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    println!("list_of_strings_closure: {:?}", list_of_strings_closure);
    println!("list_of_strings_fn: {:?}", list_of_strings_fn);

    // `()` as initializer syntax in enum.
    // The initializers are implemented as functions that take the arguments and return an instance
    // of the enum variant.
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("list_of_statuses: {:?}", list_of_statuses);
}
