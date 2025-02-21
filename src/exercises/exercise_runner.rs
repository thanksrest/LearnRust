use crate::lessons::Exercise;
use std::io::{self, Write};

pub struct ExerciseRunner;

impl ExerciseRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn run_exercise(&self, exercise: Exercise) {
        println!("\nExercise: {}", exercise.question);
        print!("Your answer: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("\nCorrect answer:");
        println!("{}", exercise.answer);
    }
}
