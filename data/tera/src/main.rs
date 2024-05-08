#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Rocket with Tera"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;

