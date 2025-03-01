use actix_web::*;
use serde::Serialize;

#[derive(Serialize)]
struct Index {
  message: String
}

#[get("/")]
async fn index() -> Result<impl Responder> {
  Ok(
    web::Json(Index {
      message: "Hello, World".to_string()
    })
  )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(index))
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
