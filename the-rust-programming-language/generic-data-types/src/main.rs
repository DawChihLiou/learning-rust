// define a `struct` with generic types
struct Point<T> {
    x: T,
    y: T,
}

// define a `enum` with generic typs
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// define a method on generic type Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// define a method with a concretely typed Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/*
 * Now we have two methods in `Point`.
 * 1. `distance_from_origin` for type `Point<f32>`
 * 2. `x` for all types where T is not type `f32`
 */ 

fn main() {
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());
}
