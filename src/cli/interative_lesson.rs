use crate::lessons::{Lesson, LessonManager};
use crate::exercises::ExerciseRunner;
use std::io::{self, Write};

pub struct InteractiveLessonCLI {
    lesson_manager: LessonManager,
    exercise_runner: ExerciseRunner,
}

impl InteractiveLessonCLI {
    pub fn new() -> Self {
        Self {
            lesson_manager: LessonManager::new(),
            exercise_runner: ExerciseRunner::new(),
        }
    }

    pub async fn run(&mut self) -> io::Result<()> {
        println!("Welcome to RustLearner Interactive CLI!");

        loop {
            let current_lesson = self.lesson_manager.current_lesson();
            println!("\nCurrent Lesson: {}", current_lesson.title());
            println!("{}", current_lesson.content());

            println!("\nOptions:");
            println!("1. Run exercise");
            println!("2. Next lesson");
            println!("3. Previous lesson");
            println!("4. Quit");

            print!("Enter your choice: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "1" => self.exercise_runner.run_exercise(current_lesson.exercise()),
                "2" => self.lesson_manager.next_lesson(),
                "3" => self.lesson_manager.previous_lesson(),
                "4" => break,
                _ => println!("Invalid option, please try again."),
            }
        }

        println!("Thank you for using RustLearner!");
        Ok(())
    }
}
