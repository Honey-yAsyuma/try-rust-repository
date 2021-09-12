pub fn libfunc() {
    let a = "test";
    println!("{}", a);
}

mod modules;

use crate::modules::my_mod::nest::ABox;
use actix_web::{get, web, App, HttpServer, Responder};
use modules::my_mod::nest::BBox as B;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    libfunc();
    modules::my_mod::modfunc();
    let ss = modules::my_mod::nest::ABox {
        contents: "tesssss",
    };
    println!("{}", ss.contents);

    let lll = ABox { contents: "llll" };
    println!("{}", lll.contents);

    let zzz = B { bcontents: "zzz" };
    println!("{}", zzz.bcontents);
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8888")?
        .run()
        .await
}
