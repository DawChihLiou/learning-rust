// defining a trait.
// trait allows abstraction across common behavior.
pub trait Draw {
    fn draw(&self);
}

//
// `Screen` has a `components` member. `components` is a vector of "trait object".
// A trait object points to both an instance of a type implementing our specified trait as well as
// a table used to look up trait methods on that type at runtime.
//
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // draw button
    }
}
