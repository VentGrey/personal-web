#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate tera;

// Working with tera
use rocket_contrib::templates::Template;

mod routes;
use crate::routes::{get, static_files};

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                static_files::file,
                get::index,
            ],
        )
        .register(catchers![errors::not_found])
}

fn main() {
    rocket().launch();
}
