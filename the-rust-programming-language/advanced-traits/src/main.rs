use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

//
// The default generic type in `Add` trait is defined as:
//
// ```
// trait Add<Rhs=Self> {
//    type Output;
//    fn add(self, rhs: Rhs) -> Self::Output;
// }
// ```
// `<Rhs=Self>`: default type paramter
//
// You’ll use default type parameters in two main ways:
//
// - To extend a type without breaking existing code
// - To allow customization in specific cases most users won’t need
//
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

//
// Fully qualified syntax for disambiguation: calling methods with the same name
//
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

//
// Using supertraits to require one trait's functionality within another trait
//
// We can define a trait that uses another trait's functionality. The another
// trait is called a supertrait. Our trait is rely on the supertrait.
use std::fmt;

// `OutlinePrint` requires `Display` trait. We can only use `OutlinePrint` trait
// on a type that implements `Display` trait.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

//
// Using the Newtype pattern to implemnet external traits on external types.
//
// The orphan rule prevents us from directly implementing a trait in an
// external crate on a type. We can use Newtype to get around the orphan
// rule.
//
// To implement an external trait on a type, We can create a wrapper struct
// that holds the type and implement the trait on the wrapper.
//
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `self.0` to access the vector in the Wrapper. Wrapper is a tuple.
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    //
    // Default generic type parameters and operator overloading
    //
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // calling methods with the same name
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // Calling an associated function(without a `self` paramter) with fully qualified syntax.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let point = Point { x: 2, y: 3 };
    point.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
