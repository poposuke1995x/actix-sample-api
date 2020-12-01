mod domains;

use actix_web::{get, web, HttpResponse, Result};
use crate::domains::entities::Post;


#[get("/post/{title}")]
async fn index(title: web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Post {
        id: 1,
        title: title.0,
        published: true
    }))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new()
        .service(index)
    )
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
