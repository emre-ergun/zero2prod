//! src/startup.rs

use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // wrap the connection with the smart pointer
    let db_pool = Data::new(db_pool);
    // wrap the email client with the smart pointer
    let email_client = Data::new(email_client);
    // Capture the connection from the surrounded environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // register the connnection as part of the application state
            .app_data(db_pool.clone())
            // register the emmail client as part of the application state
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
