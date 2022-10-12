use actix_web::{App, HttpServer};
use todo_app_benchmarks::schema::user_accounts;
use std::fs::File;
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;

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

    // let mut conn = todo_app_benchmarks::db::establish_connection();

    // let user_accounts = 
    //     todo_app_benchmarks::repositories::find_all_user_accounts(&mut conn);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = 
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

    let user_account_list = 
        user_accounts::dsl::user_accounts
            .load::<todo_app_benchmarks::models::UserAccounts>(&mut conn)
            .expect("Error loading user_accounts");

    for user_account in user_account_list {
        println!("{:?}", user_account);
    }

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
