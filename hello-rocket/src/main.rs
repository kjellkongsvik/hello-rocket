#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;

mod hello;
mod other;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/hello", routes![hello::world])
        .mount("/other", routes![other::other])
        .register(catchers![not_found])
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("not found '{}'", req.uri()))
}

fn main() {
    rocket().launch();
}
