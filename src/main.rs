#[macro_use] extern crate rocket;

mod routes;
mod util;

use util::config::load_config;
use routes::upload::upload_file;

#[launch]
fn rocket() -> _ {
    let config = load_config();

    println!("{:?}", config.secrets);
    println!("{}", config.upload_folder);
    println!("{:?}", config.allowed_extensions);

    rocket::build().mount("/", routes![upload_file])
}