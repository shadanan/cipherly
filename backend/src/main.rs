#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use std::env;

mod google;

#[get("/hello")]
fn hello(claims: google::Claims) -> &'static str {
    println!("{:?}", claims);
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let port = match env::var("PORT") {
        Ok(port) => port.parse::<i32>().unwrap(),
        Err(_) => 8000,
    };

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", port)))
        .mount("/api", routes![hello])
        .mount("/", FileServer::from("./static"))
}
