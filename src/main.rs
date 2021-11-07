use actix_web::{web, App, HttpServer};
mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .route("/technologies", web::get().to(handlers::get_technologies))
            .route("/technologies/{id}", web::get().to(handlers::get_technology_by_id))
            .route("/technologies", web::post().to(handlers::add_technology))
            .route("/technologies/{id}", web::delete().to(handlers::delete_technology))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
