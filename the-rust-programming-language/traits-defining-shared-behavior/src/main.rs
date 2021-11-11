/*
 * Traits are a way share functionality of a type with other types.
 * Similar to interfaces in other languages with some differences.
 *
 * To define a trait:
 *
 * ```
 *  pub trait TraitName {
 *      fn method_name(&self) -> ReturnType;
 *      fn another_method(&self) -> ReturnType {
 *          // default implementation
 *      }
 *  }
 * ```
 *
 * To implement a trait on a type:
 *
 * ```
 *  pub struct AType {
 *      pub name: String,
 *  }
 *
 *  impl TraitName for AType {
 *      fn method_name(&self) -> ReturnType {
 *          // implementation
 *
 *          // a method uses another method
 *          println!("{}", self.another_method());
 *      }
 *  }
 * ```
 *
 * A Trait can be used as a parameter type or a return type.
 *
 * We can use "impl Trait" syntax or trait bound syntax when
 * using a trait as parameter. We can use "+" syntax and "where" clauses
 * to express multiple trait bounds.
 *
 * We can use trait bounds to implement method conditionally, similar to
 * "overload" in some languages.
 */

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
