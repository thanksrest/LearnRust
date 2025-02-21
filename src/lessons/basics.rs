use super::{Lesson, Exercise};

pub struct VariablesAndTypes;

impl Lesson for VariablesAndTypes {
    fn title(&self) -> &str {
        "Variables and Types"
    }

    fn content(&self) -> &str {
        "Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, we must add a type annotation..."
    }

    fn exercise(&self) -> Exercise {
        Exercise {
            question: "Declare a variable `x` of type `i32` and assign it the value 5.".to_string(),
            answer: "let x: i32 = 5;".to_string(),
        }
    }
}

pub struct ControlFlow;

impl Lesson for ControlFlow {
    fn title(&self) -> &str {
        "Control Flow"
    }

    fn content(&self) -> &str {
        "Rust provides several ways to control the flow of execution in your program. The most common are if expressions and loops..."
    }

    fn exercise(&self) -> Exercise {
        Exercise {
            question: "Write an if statement that prints 'Even' if a number is even, and 'Odd' if it's odd.".to_string(),
            answer: r#"
if number % 2 == 0 {
    println!("Even");
} else {
    println!("Odd");
}"#.to_string(),
        }
    }
}
