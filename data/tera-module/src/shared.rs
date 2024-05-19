use serde::Deserialize;

#[derive(Deserialize)]
pub struct MyConfig {
    pub title: String,
}
