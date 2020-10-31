use rocket_contrib::json::JsonValue;

#[get("/ps")]
pub fn ps_processes() -> JsonValue {
    json!({ "processes": [] })
}

#[get("/ps/worker/<pid>")]
pub fn ps_worker_process(pid: u16) -> JsonValue {
    json!({ "processes": [] })
}

#[get("/ps/master/<pid>")]
pub fn ps_master_process(pid: u16) -> JsonValue {
    json!({ "processes": [] })
}

#[get("/ps/project")]
pub fn ps_project_process() -> JsonValue {
    json!({ "processes": [] })
}
