use rocket::serde::json::Json;
#[macro_use] extern crate rocket;

#[derive(serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct Index {
  message: String
}

#[get("/")]
fn index() -> Json<Index> {
  Json(Index {
    message: "Hello, World!".to_string()
  })
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index])
}
