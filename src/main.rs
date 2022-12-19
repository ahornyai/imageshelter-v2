#[macro_use] extern crate rocket;

mod routes;
mod util;

use std::path::Path;

use routes::upload::upload_file;
use util::config::CONFIG;

#[launch]
fn rocket() -> _ {
    if !Path::new(&CONFIG.upload_folder).exists() {
        std::fs::create_dir(&CONFIG.upload_folder).expect("Failed to create upload folder");
    }

    rocket::build().mount("/", routes![upload_file])
}