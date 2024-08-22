use rocket::routes;
use rocket_db_pools::{Connection, Database};

mod model;
mod schema;
mod repositories;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);
#[rocket::get("/rustaceans")]
fn get_rustaceans(db: &Connection<DbConn>) {

}
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
