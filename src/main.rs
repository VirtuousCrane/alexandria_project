#[macro_use] extern crate rocket;

use rocket::{fs::{relative, FileServer}, State};
use alexandria_project::controller::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![ping, hello, get_ip, isbn_handler])
}
