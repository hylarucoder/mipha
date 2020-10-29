use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::{PyErr, PyResult};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
struct Event {
    r#type: String,
    at: u64,
    frame: u64,
}

#[derive(Serialize, Deserialize)]
struct Profile {
    r#type: String,
    name: String,
    unit: String,
    start_value: String,
    end_value: String,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize)]
struct Frame {
    name: String,
    file: String,
    line: u64,
    col: u8,
}

#[derive(Serialize, Deserialize)]
struct ShareFrames {
    frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize)]
struct SpeedScopeStruct {
    schema: String,
    profiles: Vec<Profile>,
    shared: ShareFrames,
    active_profile_index: u8,
    exporter: String,
    name: String,
}

#[pyclass]
struct Tracer {}

pub fn make_error<E: fmt::Display + Sized>(e: E) -> PyErr {
    PyErr::new::<PyRuntimeError, _>(format!("{}", e))
}

#[pymethods]
impl Tracer {
    #[new]
    fn new() -> Self {
        Tracer {}
    }
    fn start(&mut self) -> PyResult<()> {
        Ok(())
    }
    fn stop(&mut self) -> PyResult<()> {
        Ok(())
    }
    fn trace(&mut self, pid: i32, sample_rate: u64, trace_line: &str) -> PyResult<String> {
        // Create a new PythonSpy object with the default config options
        let mut config = py_spy::Config::default();
        config.dump_locals = true;
        let mut process = py_spy::PythonSpy::new(pid, &config).map_err(make_error)?;
        println!(
            "Process {}: {}",
            process.pid,
            process.process.cmdline().map_err(make_error)?.join(" ")
        );

        println!(
            "Python v{} ({})\n",
            &process.version,
            process.process.exe().map_err(make_error)?
        );
        for i in 1..10 {
            let traces = process.get_stack_traces().map_err(make_error)?;

            for trace in traces {
                let thread_id = trace.format_threadid();
                match trace.os_thread_id.as_ref() {
                    Some(name) => {
                        println!(
                            "Thread {} ({}): \"{}\"",
                            thread_id,
                            trace.status_str(),
                            name
                        );
                    }
                    None => {
                        println!("Thread {} ({})", thread_id, trace.status_str());
                    }
                };

                for frame in &trace.frames {
                    let filename = match &frame.short_filename {
                        Some(f) => &f,
                        None => &frame.filename,
                    };
                    if frame.line != 0 {
                        if filename.ends_with(trace_line) {
                            println!("    {} ({}:{})", &frame.name, &filename, frame.line);
                        } else {
                            println!("    {} ({}:{})", &frame.name, &filename, frame.line);
                            // continue;
                        }
                    } else {
                        println!("    {} ({})", &frame.name, &filename);
                    }

                    if let Some(locals) = &frame.locals {
                        let mut shown_args = false;
                        let mut shown_locals = false;
                        for local in locals {
                            if local.arg && !shown_args {
                                println!("        {}:", "Arguments:");
                                shown_args = true;
                            } else if !local.arg && !shown_locals {
                                println!("        {}:", "Locals:");
                                shown_locals = true;
                            }

                            let repr = local.repr.as_ref().map(String::as_str).unwrap_or("?");
                            println!("            {}: {}", local.name, repr);
                        }
                    }
                }
            }
        }
        Ok("1".to_string())
    }
}

#[pymodule]
fn spy(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<Tracer>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tracer() {
        let mut tracer = Tracer {};
        tracer.trace(56417, 213, "toolbar/apps/admin.py");
        assert_eq!(1, 1);
    }
}
