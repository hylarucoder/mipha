use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PyArg {
    name: String,
    repr: String,
}

#[derive(Serialize, Deserialize)]
pub struct PyLocal {
    name: String,
    repr: String,
}

#[derive(Serialize, Deserialize)]
pub struct PyFrame {
    name: String,
    filename: String,
    line: i32,
    args: Vec<PyArg>,
    locals: Vec<PyLocal>,
}

#[derive(Serialize, Deserialize)]
pub struct PyTrace {
    thread_id: String,
    name: String,
    status: String,
    frames: Vec<PyFrame>,
}

#[derive(Serialize, Deserialize)]
pub struct PyProcess {
    pid: i32,
    version: String,
    cmdline: String,
    exe: String,
    traces: Vec<PyTrace>,
}

pub struct Tracer {}

impl Tracer {
    fn new() -> Self {
        Tracer {}
    }
    pub(crate) fn trace(&mut self, pid: i32, trace_line: &str) -> PyProcess {
        // Create a new PythonSpy object with the default config options
        let mut config = py_spy::Config::default();
        config.dump_locals = true;
        let mut py_process = py_spy::PythonSpy::new(pid, &config).unwrap();
        let mut stacktrace = PyProcess {
            pid: py_process.pid,
            version: py_process.version.to_string(),
            cmdline: py_process.process.cmdline().unwrap().join(""),
            exe: py_process.process.exe().unwrap(),
            traces: vec![],
        };
        let py_traces = py_process.get_stack_traces().unwrap();

        for py_trace in py_traces {
            let thread_id = py_trace.format_threadid();
            let mut trace;
            match py_trace.os_thread_id.as_ref() {
                Some(name) => {
                    trace = PyTrace {
                        thread_id,
                        name: name.to_string(),
                        status: py_trace.status_str().to_string(),
                        frames: vec![],
                    };
                }
                None => {
                    trace = PyTrace {
                        thread_id,
                        name: "Known".to_string(),
                        status: py_trace.status_str().to_string(),
                        frames: vec![],
                    };
                }
            };

            let mut frames: Vec<PyFrame> = vec![];

            for py_frame in &py_trace.frames {
                let filename = match &py_frame.short_filename {
                    Some(f) => &f,
                    None => &py_frame.filename,
                };
                if py_frame.line != 0 {
                    if filename.ends_with(trace_line) {
                        frames.push(PyFrame {
                            filename: filename.to_string(),
                            name: filename.to_string(),
                            line: py_frame.line,
                            args: vec![],
                            locals: vec![],
                        });
                    } else {
                        frames.push(PyFrame {
                            filename: filename.to_string(),
                            name: filename.to_string(),
                            line: py_frame.line,
                            args: vec![],
                            locals: vec![],
                        });
                    }
                }

                let mut py_args: Vec<PyArg> = vec![];
                let mut py_locals: Vec<PyLocal> = vec![];

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
                            py_args.push(PyArg {
                                name: local.name.to_string(),
                                repr: repr.to_string(),
                            })
                        }
                        if shown_locals {
                            py_locals.push(PyLocal {
                                name: local.name.to_string(),
                                repr: repr.to_string(),
                            })
                        }
                    }
                }
            }

            trace.frames.extend(frames);
        }
        return stacktrace;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tracer() {
        let mut tracer = Tracer {};
        tracer.trace(123, "toolbar/apps/admin.py");
        assert_eq!(1, 1);
    }
}
