#[rocket::get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![hello])
}

