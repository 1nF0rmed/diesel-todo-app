extern crate diesel;
extern crate diesel_todo_app;

use actix_web::{middleware, App, HttpServer};
use diesel_todo_app::routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let serv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .configure(routes)
    });
    serv.bind("0.0.0.0:8080")?.run().await
}

