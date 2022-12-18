#[macro_use] extern crate rocket;

mod routes;

use routes::upload::upload_file;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload_file])
}