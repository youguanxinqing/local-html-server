use actix_web::{web, App, HttpServer};
use clap::Parser;

mod route;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,

    #[arg(long, default_value_t = 9091)]
    port: u16,

    #[arg(long, default_value_t = String::from("static"))]
    static_dir: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(route::index))
            .route("/index", web::get().to(route::index))
    })
    .bind((args.host, args.port))?
    .run()
    .await
}
