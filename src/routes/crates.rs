use rocket::{delete, get, post, put};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;
use crate::DbConn;
use crate::model::{Crate, NewCrate, NewRustacean, Rustacean};
use crate::repositories::{CrateRepository, RustaceanRepository};

#[get("/crates")]
pub async fn search(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100)
        .await
        .map(|c| json!(c))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}

#[get("/crate/<id>")]
pub async fn get_by_id(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|c| json!(c))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}

#[post("/crate", format = "json", data = "<request>")]
pub async fn create(
    mut db: Connection<DbConn>,
    request: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, request.into_inner())
        .await
        .map(|c| Custom(Status::Created, json!(c)))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}

#[put("/crate/<id>", format = "json", data = "<request>")]
pub async fn update(
    mut db: Connection<DbConn>,
    id: i32,
    request: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, request.into_inner())
        .await
        .map(|c| json!(c))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}

#[delete("/crate/<id>")]
pub async fn delete(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|c| json!(c))
        .map_err(|_| Custom(Status::InternalServerError, json!("error!")))
}
