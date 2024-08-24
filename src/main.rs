use crate::routes::rustaceans::get_rustaceans;
use rocket::routes;
use rocket_db_pools::Database;
use routes::rustaceans::{create, delete, get_by_id, update};

mod model;
mod repositories;
mod routes;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![get_rustaceans, get_by_id, create, update, delete],
        )
        .attach(DbConn::init())
        .launch()
        .await;
}
