mod modules;

use actix_web::{get, web, App, HttpServer, Responder};
use modules::psql;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    psql::init();
    HttpServer::new(|| App::new().service(index))
        // .bind("0.0.0.0:8888")?
        .bind("localhost:8888")?
        .run()
        .await
}
