use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::db;
use crate::models;
use crate::repositories;
use crate::protocols;

#[get("/{user_id}/task")]
async fn list(path: web::Path<(String,)>) -> impl Responder {
    // show todo all
    let user_id = &path.into_inner().0.to_string();
    let mut conn = db::establish_connection();
    let tasks = repositories::find_all_tasks_by_user_id(&mut conn, user_id);
    let response = protocols::FindAllTasksResponseJson {
        id: Uuid::new_v4().to_hyphenated().to_string(),
        user_id: user_id.to_string(),
        tasks: tasks,
        create_at: Utc::now(),
        time_zone: String::from("utc")
    };
    let json = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok().body(json)
}

#[post("/{user_id}/task/")]
async fn post(path: web::Path<(String,)>, request_json: web::Json<protocols::PostTaskRequestJson>) -> impl Responder {
    // post task
    let user_id = path.into_inner().0;
    let task_request = request_json.0;
    
    let task_id = &Uuid::new_v4().to_hyphenated().to_string()[0..12].to_string();
    let create_at = Utc::now();

    let task = models::Tasks {
        id: task_id.to_string(),
        user_id: user_id.to_string(),
        title: task_request.title,
        close: false,
        create_at: create_at,
        modify_at: None,
        close_at: None
    };
    let mut conn = db::establish_connection();
    let inserted = repositories::insert_task(&mut conn, task);
    let response = protocols::PostTaskResponseJson {
        id: Uuid::new_v4().to_hyphenated().to_string(),
        user_id: user_id.to_string(),
        task_id: task_id.to_string(),
        title: inserted.title,
        create_at: create_at,
        time_zone: "utc".to_string()
    };
    let json = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok().body(json)
}

#[get("/{user_id}/task/{id}")]
async fn details(path: web::Path<(String,String)>) -> impl Responder {
    // show details
    let (user_id, task_id) = path.into_inner();
    let mut conn = db::establish_connection();
    let task = repositories::find_task_by_id(&mut conn, &user_id, &task_id);
    match task {
        Some(t) => {
            let response = protocols::FindTaskByIdResponseJson {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                user_id: user_id.to_string(),
                task: t,
                create_at: Utc::now(),
                time_zone: String::from("utc")
            };
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().body(json)
        },
        None => HttpResponse::NotFound().body(format!("Not found task {}", task_id.to_string()))
    }
}

#[put("/{user_id}/task/{id}")]
async fn update(path: web::Path<(String, String)>, request_json: web::Json<protocols::UpdateTaskRequestJson>) -> impl Responder {
    // update task
    let (user_id, task_id) = path.into_inner();
    let update_request = request_json.0;

    let mut conn = db::establish_connection();
    let task = repositories::find_task_by_id(&mut conn, &user_id, &task_id);
    match task {
        Some(t) => {
            let updated = 
                repositories::update_task(
                    &mut conn, 
                    models::Tasks {
                        id: t.id,
                        user_id: t.user_id,
                        title: update_request.title,
                        close: t.close,
                        create_at: t.create_at,
                        modify_at: Some(Utc::now()),
                        close_at: t.close_at
                    }
                );

            let response = protocols::UpdateTaskResponseJson {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                user_id: user_id,
                task: updated,
                create_at: Utc::now(),
                time_zone: String::from("utc")
            };
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().body(json)
        },
        None => HttpResponse::NotFound().body(format!("Not found task {}", task_id.to_string()))
    }
}

#[delete("/{user_id}/task/{id}")]
async fn delete(path: web::Path<(String, String)>) -> impl Responder { 
    // delete task
    let (user_id, task_id) = path.into_inner();
    let mut conn = db::establish_connection();
    
    let tasks_len = repositories::find_all_tasks_by_user_id(&mut conn, &user_id).len();
    if tasks_len < 1 {
        HttpResponse::BadRequest().body("Not exists users tasks.")
    } else {
        let deleted = repositories::delete_task_by_id(&mut conn, &user_id, &task_id);
        if (tasks_len - 1) == deleted {
            let response = protocols::DeleteTaskResponseJson {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                user_id: user_id,
                task_id: task_id,
                create_at: Utc::now(),
                time_zone: String::from("utc")
            };
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().body(json)
        } else {
            HttpResponse::NotFound().body(format!("Not found task {}.", task_id))
        }
    }
}