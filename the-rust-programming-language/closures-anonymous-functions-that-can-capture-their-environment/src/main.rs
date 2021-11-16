use std::thread;
use std::time::Duration;

fn main() {
    /*
     * Closures are anonymous functions. They can capture variables
     * from its environment (lexical scope).
     */
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // closure types can be explicit or implicit. Compiler will infer
    // if the types are absent.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {}, situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!(
                "Today, run for {} minites!",
                expensive_result.value(intensity)
            );
        }
    }
}

/*
 * Storing closures using generic parameters and the `Fn` traits.
 *
 * All closures implement at least one of the traits from standard library
 * to define how they capture a value from its environment:
 *
 * - `Fn`: borrows values from the environment immutably
 * - `FnMut`: borrow values mutably
 * - `FnOnce`: move (take ownership of) values from the environment into the clsure
 *
 */
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
