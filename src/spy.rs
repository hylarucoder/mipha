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
        let mut process = py_spy::PythonSpy::new(pid, &config).unwrap();
        let mut py_process = PyProcess {
            pid: process.pid,
            version: process.version.to_string(),
            cmdline: process.process.cmdline().unwrap().join(""),
            exe: process.process.exe().unwrap(),
            traces: vec![],
        };
        let traces = process.get_stack_traces().unwrap();

        for trace in traces {
            let thread_id = trace.format_threadid();
            let mut py_trace;
            match trace.os_thread_id.as_ref() {
                Some(name) => {
                    py_trace = PyTrace {
                        thread_id,
                        name: name.to_string(),
                        status: trace.status_str().to_string(),
                        frames: vec![],
                    };
                }
                None => {
                    py_trace = PyTrace {
                        thread_id,
                        name: "Known".to_string(),
                        status: trace.status_str().to_string(),
                        frames: vec![],
                    };
                }
            };

            let mut py_frames: Vec<PyFrame> = vec![];

            for frame in &trace.frames {
                let filename = match &frame.short_filename {
                    Some(f) => &f,
                    None => &frame.filename,
                };
                let mut py_frame;
                // 不处理 native extension
                if frame.line == 0 {
                    continue;
                }
                // 匹配规则
                if filename.ends_with(trace_line) {
                    py_frame = PyFrame {
                        filename: filename.to_string(),
                        name: filename.to_string(),
                        line: frame.line,
                        args: vec![],
                        locals: vec![],
                    };
                } else {
                    py_frame = PyFrame {
                        filename: filename.to_string(),
                        name: filename.to_string(),
                        line: frame.line,
                        args: vec![],
                        locals: vec![],
                    };
                }

                let mut py_args: Vec<PyArg> = vec![];
                let mut py_locals: Vec<PyLocal> = vec![];

                if let Some(locals) = &frame.locals {
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
                            });
                        }
                    }
                    py_frame.locals.extend(py_locals);
                    py_frame.args.extend(py_args);
                    py_trace.frames.push(py_frame);
                }
            }

            py_process.traces.push(py_trace);
        }
        return py_process;
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
