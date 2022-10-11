use actix_web::{App, HttpServer};
use config::Config;
use std::fs::File;

mod adapter;

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
            File::create("application.log").unwrap(),
        ),
    ])
    .unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    init_logger();

    let config = 
        Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()
            .unwrap();

    let host: String = config.get("host").unwrap();
    let port: u16 = config.get("port").unwrap();

    log::info!("Starting server on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(adapter::routes::list)
            .service(adapter::routes::post)
            .service(adapter::routes::details)
            .service(adapter::routes::update)
            .service(adapter::routes::delete)
    })
    .bind((host, port))
    .expect(&format!("Failed open port to {}", port))
    .run()
    .await
}
