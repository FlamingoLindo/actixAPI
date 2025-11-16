mod middleware;
mod models;
mod routes;
mod steam;
use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, middleware::Logger};
use routes::{config::config, health_route::health_checker_handler};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("Database URL not found!");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Database connection was a success!");
            pool
        }
        Err(err) => {
            println!(
                "There has been an error during the database connection! {:?}",
                err
            );
            std::process::exit(1);
        }
    };

    println!("Server started!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(AppState { db: pool.clone() }))
            .service(health_checker_handler)
            .configure(config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
