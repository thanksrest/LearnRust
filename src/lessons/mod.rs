mod basics;
mod advanced;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Exercise {
    pub question: String,
    pub answer: String,
}

pub trait Lesson {
    fn title(&self) -> &str;
    fn content(&self) -> &str;
    fn exercise(&self) -> Exercise;
}

pub struct LessonManager {
    lessons: Vec<Box<dyn Lesson>>,
    current_index: usize,
}

impl LessonManager {
    pub fn new() -> Self {
        let lessons: Vec<Box<dyn Lesson>> = vec![
            Box::new(basics::VariablesAndTypes),
            Box::new(basics::ControlFlow),
            Box::new(advanced::Ownership),
            Box::new(advanced::Traits),
        ];

        Self {
            lessons,
            current_index: 0,
        }
    }

    pub fn current_lesson(&self) -> &dyn Lesson {
        self.lessons[self.current_index].as_ref()
    }

    pub fn next_lesson(&mut self) {
        if self.current_index < self.lessons.len() - 1 {
            self.current_index += 1;
        }
    }

    pub fn previous_lesson(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }
}
