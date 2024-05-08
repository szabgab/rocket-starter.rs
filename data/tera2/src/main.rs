#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};
use rocket::{fairing::AdHoc, State};
use serde::Deserialize;


#[derive(Deserialize)]
struct MyConfig {
    title: String,
}



#[get("/")]
fn index(config: &State<MyConfig>) -> Template {
    Template::render("index", context! {
        title: &config.title
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
        .attach(AdHoc::config::<MyConfig>())
        .register("/", catchers![not_found])

}

#[cfg(test)]
mod tests;

