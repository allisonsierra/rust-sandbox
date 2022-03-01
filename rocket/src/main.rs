#[macro_use]
extern crate rocket;

mod groups;
mod json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!\n"
}

#[get("/roles")]
fn roles() -> &'static str {
    "Roles placeholder\n"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, roles])
}
