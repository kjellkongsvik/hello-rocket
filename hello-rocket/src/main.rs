#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/world")]
fn index() -> &'static str {
    "Hessllo, world!"
}

fn main() {
    rocket::ignite().mount("/hello", routes![index]).launch();
}
