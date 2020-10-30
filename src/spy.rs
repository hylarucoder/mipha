use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::{PyErr, PyResult};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fmt;

#[derive(Serialize, Deserialize)]
struct Arg {
    name: String,
    repr: String,
}

#[derive(Serialize, Deserialize)]
struct Local {
    name: String,
    repr: String,
}

#[derive(Serialize, Deserialize)]
struct Frame {
    name: String,
    filename: String,
    line: i32,
    args: Vec<Arg>,
    locals: Vec<Local>,
}

#[derive(Serialize, Deserialize)]
struct Trace {
    thread_id: String,
    name: String,
    status: String,
    frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize)]
struct StackTraces {
    pid: i32,
    version: String,
    cmdline: String,
    exe: String,
    start_at: Option<String>,
    end_at: Option<String>,
    traces: Vec<Trace>,
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
        let mut py_process = py_spy::PythonSpy::new(pid, &config).map_err(make_error)?;
        let mut stacktrace = StackTraces {
            pid: py_process.pid,
            version: py_process.version.to_string(),
            cmdline: py_process.process.cmdline().map_err(make_error)?.join(""),
            exe: py_process.process.exe().map_err(make_error)?,
            start_at: None,
            end_at: None,
            traces: vec![],
        };
        let py_traces = py_process.get_stack_traces().map_err(make_error)?;

        for py_trace in py_traces {
            let thread_id = py_trace.format_threadid();
            let mut trace;
            match py_trace.os_thread_id.as_ref() {
                Some(name) => {
                    trace = Trace {
                        thread_id,
                        name: name.to_string(),
                        status: py_trace.status_str().parse()?,
                        frames: vec![],
                    };
                }
                None => {
                    trace = Trace {
                        thread_id,
                        name: "Known".parse()?,
                        status: py_trace.status_str().parse()?,
                        frames: vec![],
                    };
                }
            };

            let mut frames: Vec<Frame> = vec![];

            for py_frame in &py_trace.frames {
                let filename = match &py_frame.short_filename {
                    Some(f) => &f,
                    None => &py_frame.filename,
                };
                if py_frame.line != 0 {
                    if filename.ends_with(trace_line) {
                        frames.push(Frame {
                            filename: filename.to_string(),
                            name: filename.to_string(),
                            line: py_frame.line,
                            args: vec![],
                            locals: vec![],
                        });
                    }
                }

                let mut py_args: Vec<Arg> = vec![];
                let mut py_locals: Vec<Local> = vec![];

                if let Some(locals) = &py_frame.locals {
                    let mut shown_args = false;
                    let mut shown_locals = false;
                    for local in locals {
                        if local.arg && !shown_args {
                            shown_args = true;
                        } else if !local.arg && !shown_locals {
                            shown_locals = true;
                        }
                        let repr = local.repr.as_ref().map(String::as_str).unwrap_or("?");
                        if shown_args {
                            py_args.push(Arg {
                                name: local.name.to_string(),
                                repr: repr.parse()?,
                            })
                        }
                        if shown_locals {
                            py_locals.push(Local {
                                name: local.name.to_string(),
                                repr: repr.parse()?,
                            })
                        }
                    }
                }
            }

            trace.frames.extend(frames);
        }
        let result = to_string(&stacktrace).map_err(make_error)?;
        Ok(result)
    }
}

#[pymodule]
fn spy(_py: Python, m: &PyModule) -> PyResult<()> {
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
        tracer.trace(123, 213, "toolbar/apps/admin.py");
        assert_eq!(1, 1);
    }
}
