use actix_web::{get, post, put, delete, Responder, HttpResponse};

#[get("/")]
async fn list() -> impl Responder {
    // show todo all
    HttpResponse::Ok().body("Hello world!")
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