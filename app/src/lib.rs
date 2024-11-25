pub mod configuration;
pub mod routes;
pub mod startup;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}
