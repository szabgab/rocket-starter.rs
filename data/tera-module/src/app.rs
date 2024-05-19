use crate::shared::MyConfig;
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};

pub fn routes() -> Vec<Route> {
    routes![index]
}

#[get("/")]
pub fn index(config: &State<MyConfig>) -> Template {
    Template::render(
        "index",
        context! {
            title: format!("App {}", &config.title)
        },
    )
}

#[cfg(test)]
mod test_app;
