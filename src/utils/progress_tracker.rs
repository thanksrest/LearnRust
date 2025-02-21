use std::collections::HashMap;

pub struct ProgressTracker {
    completed_lessons: HashMap<String, bool>,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            completed_lessons: HashMap::new(),
        }
    }

    pub fn mark_completed(&mut self, lesson_title: &str) {
        self.completed_lessons.insert(lesson_title.to_string(), true);
    }

    pub fn is_completed(&self, lesson_title: &str) -> bool {
        *self.completed_lessons.get(lesson_title).unwrap_or(&false)
    }

    pub fn progress_percentage(&self, total_lessons: usize) -> f32 {
        let completed = self.completed_lessons.values().filter(|&&v| v).count();
        (completed as f32 / total_lessons as f32) * 100.0
    }
}
