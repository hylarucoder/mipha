use log::{debug, info, trace};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3_log::{Caching, Logger};
use std::collections::HashMap;

struct Record {
    timestamp: u64,
    typ: String,
    filename: String,
    line: u64,
    name: String,
}

struct Frame {
    name: String,
    file: String,
    line: u64,
    col: u8,
}

struct Event {
    r#type: String,
    at: u64,
    frame: u64,
}

#[pyclass]
struct SpRecorder {
    records: Vec<Record>,
}

#[pymethods]
impl SpRecorder {
    #[new]
    fn new() -> Self {
        SpRecorder { records: vec![] }
    }

    fn start(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn stop(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn append_record(
        &mut self,
        timestamp: u64,
        typ: String,
        filename: String,
        line: u64,
        name: String,
    ) -> PyResult<()> {
        let record = Record {
            timestamp,
            typ,
            filename,
            line,
            name,
        };
        self.records.push(record);
        Ok(())
    }

    fn len(&mut self) -> PyResult<usize> {
        Ok(self.records.len())
    }

    fn _make_speed_scope_dict(&mut self) {
        let mut events: Vec<Event> = Vec::new();
        let mut frames: Vec<Frame> = Vec::new();
        let mut frame_cache: HashMap<&str, u64> = Default::default();
        // while self.records[0][1] == "C" {
        //     self.records.pop(0)
        // }
        //
        // while self.records[-1][1] == "O" {
        //     self.records.pop(-1)
        // }
        for record in &self.records {
            let filename = record.filename.clone();
            let line = record.line.clone();
            let name = record.name.clone();
            let timestamp = record.timestamp.clone();
            let r#type = record.typ.clone();
            let key_items = [filename, line.to_string(), name];
            let key = String::from(key_items.join("-"));
            if !frame_cache.contains_key::<str>(&key) {
                frame_cache.insert(&key, frames.len() as u64);
                let frame = Frame {
                    name,
                    file: filename,
                    line,
                    col: 1,
                };
                frames.push(frame);
            }
            match frame_cache.get::<str>(&key) {
                Some(frame_cache_count) => {
                    let event = Event {
                        r#type,
                        at: timestamp,
                        frame: *frame_cache_count,
                    };
                    events.push(event);
                }
                None => {}
            }
        }
    }

    fn export_to_json(&mut self, filename: &str) -> PyResult<String> {
        Ok(format!("--"))
    }
}

#[pymodule]
fn speedscope(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<SpRecorder>()?;

    Ok(())
}
