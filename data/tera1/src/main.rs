#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Rocket with Tera"
    })
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", context! {
        // Currently the title is set in the template
        //title: "404 Not Found"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}

#[cfg(test)]
mod tests;

