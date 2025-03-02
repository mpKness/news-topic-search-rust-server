use actix_web::{web, App, HttpServer};
use actix_files as fs;
use tera::Tera;

mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*")).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(tera.clone())
            .route("/", web::get().to(templates::index))
            .route("/", web::post().to(templates::handle_post_topic))
            .route("/about", web::get().to(templates::about))
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
