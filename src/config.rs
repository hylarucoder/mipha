use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use std::collections::HashMap;
use std::env;

pub fn from_env() -> Config {
    let environment = Environment::active().expect("No environment found");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    Config::build(environment)
        .environment(environment)
        .port(port)
        .finalize()
        .unwrap()
}
