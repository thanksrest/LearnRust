use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::lessons::LessonManager;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to RustLearner!")
}

async fn get_lesson(lesson_manager: web::Data<LessonManager>) -> impl Responder {
    let lesson = lesson_manager.current_lesson();
    HttpResponse::Ok().json(lesson)
}

pub async fn run_server() -> std::io::Result<()> {
    let lesson_manager = web::Data::new(LessonManager::new());

    HttpServer::new(move || {
        App::new()
            .app_data(lesson_manager.clone())
            .route("/", web::get().to(index))
            .route("/lesson", web::get().to(get_lesson))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
