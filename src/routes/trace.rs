use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};

use crate::spy::{PyProcess, Tracer};
use sysinfo::{ProcessExt, System, SystemExt};

#[get("/ps")]
pub fn ps_processes() -> JsonValue {
    let sys = System::new_all();
    let mut processes = vec![];
    let mut tracer = Tracer {};
    for (pid, process) in sys.get_processes() {
        let mut is_target_process = false;
        for cmd in process.cmd() {
            if cmd.contains("gunicorn") {
                is_target_process = true;
                println!("pid -->{}", *pid);
                let process = tracer.trace(*pid, "toolbar/apps/admin.py");
                processes.push(process)
            }
        }
    }

    json!({ "processes": processes })
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
