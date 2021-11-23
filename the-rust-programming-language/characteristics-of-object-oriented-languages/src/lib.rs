//
// Object Oriented Programming characteristics
//
// - Encapsulation that hides implementation details.
//      We can use `struct`,`impl`, and `pub` to create public API for a struct.
// - Inheritance as a type system and as code sharing
//      We can't inherit from another `struct`. We use `trait` bounds to impose constrains on types
//      must provide. This is sometimes called bounded parametric polymorphism.
//
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
