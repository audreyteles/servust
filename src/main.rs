#[macro_use]
extern crate rocket;

use std::fs;
use rocket::fs::FileServer;
use std::path::Path;

#[launch]
fn rocket() -> _ {
    if !Path::new("servust/static").exists() {
        let _ = fs::create_dir("servust");
        let _ = fs::create_dir("servust/static");
    }

    rocket::build()
        .mount("/", FileServer::from("servust/static"))
}
