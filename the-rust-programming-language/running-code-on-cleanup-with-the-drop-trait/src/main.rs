//
// `Drop` trait allow you to describe the behavior when a value is about to go out of scope.
//
//
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    let e = CustomSmartPointer {
        data: String::from("another stuff"),
    };
    // droping a value before it goes out of scope.
    // calling destructor `e.drop()` is not allowed. Instead, use `std::mem::drop` to drop
    // explicitly.
    drop(e);
    println!("CustomSmartPointers created");
}
// variables are dropped in the reverse order of their creation.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // `drop()` runs when a value goes out of scope
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}
