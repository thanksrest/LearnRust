use rust_learner::lessons::{LessonManager, Lesson};
use rust_learner::exercises::ExerciseRunner;
use rust_learner::utils::progress_tracker::ProgressTracker;

#[test]
fn test_lesson_manager() {
    let mut manager = LessonManager::new();
    assert_eq!(manager.current_lesson().title(), "Variables and Types");
    
    manager.next_lesson();
    assert_eq!(manager.current_lesson().title(), "Control Flow");
    
    manager.previous_lesson();
    assert_eq!(manager.current_lesson().title(), "Variables and Types");
}

#[test]
fn test_progress_tracker() {
    let mut tracker = ProgressTracker::new();
    tracker.mark_completed("Variables and Types");
    
    assert!(tracker.is_completed("Variables and Types"));
    assert!(!tracker.is_completed("Control Flow"));
    
    assert_eq!(tracker.progress_percentage(4), 25.0);
}
