
#[get("/world")]
pub fn world() -> &'static str {
    "Hello, world!"
}


#[cfg(test)]
mod test_world {
    extern crate rocket;
    use rocket::local::Client;

    #[test]
    fn world() {
        let rocket = rocket::ignite().mount("/", routes![super::world]);
        let client = Client::new(rocket).unwrap();
        let mut response = client.get("/world").dispatch();
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}