use actix_web::{web, get, post, put, delete, Responder, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::db;
use crate::models;
use crate::repositories;
use crate::protocols;
use crate::utils;

#[get("/{user_id}/task")]
async fn list(path: web::Path<(String,)>) -> impl Responder {
    let user_id = &path.into_inner().0.to_string();
    let mut conn = db::establish_connection();
    match repositories::find_all_tasks_by_user_id(&mut conn, user_id) {
        Ok(tasks) => {
            let response = protocols::FindAllTasksResponseJson {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                user_id: user_id.to_string(),   
                tasks: tasks.into_iter().map(|t|t.id).collect::<Vec<_>>(),
                create_at: Utc::now(),
                time_zone: String::from("utc")
            };
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().body(json)
        }
        Err(err) => {
            log::error!(
                "An database error occurred while creating a user account. error_msg={}, user_id={}", 
                err.to_string(), 
                user_id.to_string()
            );
            HttpResponse::InternalServerError().body("A server error has occurred.")
        }
    }

}

#[post("/user/create")]
async fn create_user_account() -> impl Responder {
    let user_id = &utils::rdm_alphabet(12);
    let create_at = Utc::now();

    let user_account = models::UserAccounts {
        id: user_id.to_string(),
        close: false,
        create_at: create_at,
        close_at: None
    };
    let mut conn = db::establish_connection();
    match repositories::insert_user_account(&mut conn, user_account) {
        Ok(_) => {
            let response = protocols::CreateUserAccountResponseJson {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                user_id: user_id.to_string(),
                create_at: create_at,
                time_zone: String::from("utc")
            };
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().body(json)
        }
        Err(err) => {
            log::error!(
                "An database error occurred while creating a user account. error_msg={}, user_id={}", 
                err.to_string(), 
                user_id.to_string()
            );
            HttpResponse::InternalServerError().body("A server error has occurred.")
        }
    }
}

#[post("/{user_id}/task")]
async fn post(path: web::Path<(String,)>, request_json: web::Json<protocols::PostTaskRequestJson>) -> impl Responder {
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
    match repositories::insert_task(&mut conn, task) {
        Ok(inserted) => {
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
        Err(err) => {
            match err {
                diesel::result::Error::NotFound => {
                    log::warn!("A reference to a non-existent task has occurred. user_id={}, task_id={}",
                        user_id.to_string(),
                        task_id.to_string()
                    );
                    HttpResponse::NotFound().body("No task exists.")
                }
                _ => {
                    log::error!(
                        "An error occurred while creating a task. error_msg={}, user_id={}, task_id={}", 
                        err.to_string(), 
                        user_id.to_string(),
                        task_id.to_string()
                    );
                    HttpResponse::InternalServerError().body("A server error has occurred.")
                }
            }
        }
    }
}

#[get("/{user_id}/task/{id}")]
async fn details(path: web::Path<(String,String)>) -> impl Responder {
    let (user_id, task_id) = path.into_inner();
    let mut conn = db::establish_connection();
    match repositories::find_task_by_id(&mut conn, &user_id, &task_id) {
        Ok(task) if task.is_some() => {
            let response = protocols::FindTaskByIdResponseJson {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                user_id: user_id.to_string(),
                task: task.unwrap(),
                create_at: Utc::now(),
                time_zone: String::from("utc")
            };
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().body(json)
        }
        Err(err) => {
            log::error!(
                "An error occurred while creating a task. error_msg={}, user_id={}, task_id={}", 
                err.to_string(), 
                user_id.to_string(),
                task_id.to_string()
            );
            HttpResponse::InternalServerError().body("A server error has occurred.")
        } 
        _ => {
            log::warn!("A reference to a non-existent task has occurred. user_id={}, task_id={}",
                user_id.to_string(),
                task_id.to_string()
            );
            HttpResponse::NotFound().body(format!("Not found task {}", task_id.to_string()))
        }
    }
}

#[put("/{user_id}/task/{id}")]
async fn update(path: web::Path<(String, String)>, request_json: web::Json<protocols::UpdateTaskRequestJson>) -> impl Responder {
    let (user_id, task_id) = path.into_inner();
    let update_request = request_json.0;

    let mut conn = db::establish_connection();
    match repositories::find_task_by_id(&mut conn, &user_id, &task_id) {
        Ok(finding) if finding.is_some() => {
            let found = finding.unwrap();
            let updating = repositories::update_task(
                &mut conn, 
                models::Tasks {
                    id: found.id,
                    user_id: found.user_id,
                    title: update_request.title,
                    close: found.close,
                    create_at: found.create_at,
                    modify_at: Some(Utc::now()),
                    close_at: found.close_at
                }
            );
            match updating {
                Ok(updated) => {
                    let response = protocols::UpdateTaskResponseJson {
                        id: Uuid::new_v4().to_hyphenated().to_string(),
                        user_id: user_id,
                        task: updated,
                        create_at: Utc::now(),
                        time_zone: String::from("utc")
                    };
                    let json = serde_json::to_string(&response).unwrap();
                    HttpResponse::Ok().body(json)
                }
                Err(err) => {
                    log::error!(
                        "An error occurred while creating a task. error_msg={}, user_id={}, task_id={}", 
                        err.to_string(), 
                        user_id,
                        task_id
                    );
                    HttpResponse::InternalServerError().body("A server error has occurred.")
                }
            }
        }
        Err(err) => {
            log::error!(
                "An error occurred while creating a task. error_msg={}, user_id={}, task_id={}", 
                err.to_string(), 
                user_id,
                task_id
            );
            HttpResponse::InternalServerError().body("A server error has occurred.")
        }
        _ => {
            log::warn!("A reference to a non-existent task has occurred. user_id={}, task_id={}",
                user_id.to_string(),
                task_id.to_string()
            );
            HttpResponse::NotFound().body(format!("Not found task {}", task_id.to_string()))
        }
    }
}

#[delete("/{user_id}/task/{id}")]
async fn delete(path: web::Path<(String, String)>) -> impl Responder { 
    let (user_id, task_id) = path.into_inner();

    let mut conn = db::establish_connection();
    match repositories::delete_task_by_id(&mut conn, &user_id, &task_id) {
        Ok(_) => {
            let response = protocols::DeleteTaskResponseJson {
                id: Uuid::new_v4().to_hyphenated().to_string(),
                user_id: user_id,
                task_id: task_id,
                create_at: Utc::now(),
                time_zone: String::from("utc")
            };
            let json = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().body(json)
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                log::warn!("A reference to a non-existent task has occurred. user_id={}, task_id={}",
                    user_id.to_string(),
                    task_id.to_string()
                );
                HttpResponse::NotFound().body(format!("Not found task {}", task_id.to_string()))
            }
            _ => {
                log::error!(
                    "An error occurred while creating a task. error_msg={}, user_id={}, task_id={}", 
                    err.to_string(), 
                    user_id,
                    task_id
                );
                HttpResponse::InternalServerError().body("A server error has occurred.")
            }
        }
    }
}