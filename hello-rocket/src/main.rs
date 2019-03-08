#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
mod hello;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/hello", routes![hello::world])
}

fn main() {
    rocket().launch();
}
