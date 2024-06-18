use actix_web::{web, get, post, put, delete, HttpResponse};
use crate::{models::events::{Event, NewEvent}, repository::database::Database};

#[get("/events")]
async fn get_events(db: web::Data<Database>) -> HttpResponse {
    let events = db.get_events();
    HttpResponse::Ok().json(events)
}

#[get("/event/{id}")]
async fn get_event(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let event = db.get_event(path.into_inner());
    match event {
        None => { HttpResponse::NotFound().body("Not Found") }
        Some(event) => { HttpResponse::Ok().json(event) }
    }
}

#[post("/event")]
async fn create_event(db: web::Data<Database>, event: web::Json<NewEvent>) -> HttpResponse {
    let event = db.create_event(event.into_inner());
    match event {
        Ok(event) => { HttpResponse::Ok().json(event) }
        Err(_) => { HttpResponse::InternalServerError().body("Internal Server Error.") }
    }
}

#[put("/event")]
async fn update_event(db: web::Data<Database>, event: web::Json<Event>) -> HttpResponse {
    let event = db.update_event(event.into_inner());
    match event {
        Ok(event) => { HttpResponse::Ok().json(event) }
        Err(_) => { HttpResponse::InternalServerError().body("Internal Server Error") }
    }
}

#[delete("/event/{id}")]
async fn delete_event(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let event = db.delete_event(path.into_inner());
    match event {
        Ok(event) => { HttpResponse::Ok().json(event) }
        Err(_) => { HttpResponse::InternalServerError().body("Internal Server Error") }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_events)
            .service(get_event)
            .service(create_event)
            .service(update_event)
            .service(delete_event)
    );
}