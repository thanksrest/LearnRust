use super::{Lesson, Exercise};

pub struct Ownership;

impl Lesson for Ownership {
    fn title(&self) -> &str {
        "Ownership"
    }

    fn content(&self) -> &str {
        "Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector..."
    }

    fn exercise(&self) -> Exercise {
        Exercise {
            question: "Write a function that takes ownership of a String and returns its length.".to_string(),
            answer: r#"
fn string_length(s: String) -> usize {
    s.len()
}"#.to_string(),
        }
    }
}

pub struct Traits;

impl Lesson for Traits {
    fn title(&self) -> &str {
        "Traits"
    }

    fn content(&self) -> &str {
        "A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way..."
    }

    fn exercise(&self) -> Exercise {
        Exercise {
            question: "Define a trait called 'Printable' with a method 'print' that takes no arguments and returns nothing.".to_string(),
            answer: r#"
trait Printable {
    fn print(&self);
}"#.to_string(),
        }
    }
}
