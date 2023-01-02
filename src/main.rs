#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use alexandria_project::{controller::*, common::RequestClient};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(RequestClient::new())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![ping, hello, get_ip, isbn_handler])
}
