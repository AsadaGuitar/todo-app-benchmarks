use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::db;
use crate::repositories;
use crate::protocols;

#[get("/{user_id}")]
async fn list(path: web::Path<(String,)>) -> impl Responder {
    // show todo all
    let user_id = &path.into_inner().0;
    let mut conn = db::establish_connection();
    let tasks = repositories::find_all_tasks_by_user_id(&mut conn, user_id);
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let response = protocols::SelectAllResponse {
        id: uuid,
        user_id: user_id.to_string(),
        tasks: tasks,
        create_at: Utc::now(),
        time_zone: String::from("utc")
    };
    let json = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok().body(json)
}

#[post("/task")]
async fn post() -> impl Responder {
    // post task
    HttpResponse::Ok().body("Hello world!")
}
    
#[get("/task/{id}")]
async fn details() -> impl Responder {
    // show details
    HttpResponse::Ok().body("Hello world!")
}
    
#[put("/task/{id}")]
async fn update() -> impl Responder {
    // update task
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/task/{id}")]
async fn delete() -> impl Responder { 
    // delete task
    HttpResponse::Ok().body("Hello world!")
}