use actix_web::{App, HttpServer};
use std::fs::File;
use dotenv::dotenv;
use std::env;

fn init_logger(){
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Warn,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create("./logs/application.log").unwrap(),
        ),
    ])
    .unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_logger();

    let host: String = env::var("TODO_APP_HOST").expect("TODO_APP_HOST must be set");
    let port: u16 = env::var("TODO_APP_PORT").expect("TODO_APP_PORT must be set").parse().unwrap();

    log::info!("Starting server on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(todo_app_benchmarks::routes::list)
            .service(todo_app_benchmarks::routes::post)
            .service(todo_app_benchmarks::routes::details)
            .service(todo_app_benchmarks::routes::update)
            .service(todo_app_benchmarks::routes::delete)
    })
    .bind((host, port))
    .expect(&format!("Failed open port to {}", port))
    .run()
    .await
}
