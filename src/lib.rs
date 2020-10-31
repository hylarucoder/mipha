#![feature(proc_macro_hygiene, decl_macro)]

mod config;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_cors;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

use dotenv::dotenv;

use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;

mod routes;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![
                routes::trace::ps_processes,
                routes::trace::ps_worker_process,
                routes::trace::ps_master_process,
                routes::trace::ps_project_process,
            ],
        )
        .attach(cors_fairing())
        .register(catchers![not_found])
}
