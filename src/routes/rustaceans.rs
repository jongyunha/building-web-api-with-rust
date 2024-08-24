use crate::model::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepository;
use crate::DbConn;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put};
use rocket_db_pools::Connection;

#[get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db, 100)
        .await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}

#[get("/rustacean/<id>")]
pub async fn get_by_id(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db, id)
        .await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}

#[post("/rustacean", format = "json", data = "<request>")]
pub async fn create(
    mut db: Connection<DbConn>,
    request: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, request.into_inner())
        .await
        .map(|r| Custom(Status::Created, json!(r)))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}

#[put("/rustacean/<id>", format = "json", data = "<request>")]
pub async fn update(
    mut db: Connection<DbConn>,
    id: i32,
    request: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, request.into_inner())
        .await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}

#[delete("/rustacean/<id>")]
pub async fn delete(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id)
        .await
        .map(|r| json!(r))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}
