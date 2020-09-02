use log::info;
use pyo3::create_exception;
use pyo3::exceptions::Exception;
use pyo3::prelude::*;
use pyo3::{PyErr, PyResult};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Result};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// impl fmt::Display for serde_json::Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Oh no!")
//     }
// }
#[derive(Debug)]
enum CustomError {
    JSON(serde_json::Error),
}

impl From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError::JSON(error)
    }
}

// create_exception!(mymodule, CustomError, Exception);

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

#[derive(Serialize, Deserialize)]
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
    fn _make_speed_scope_dict(&mut self) -> Result<String> {
        let mut events: Vec<Event> = Vec::new();
        let mut frames: Vec<Frame> = Vec::new();
        let mut frame_cache: HashMap<String, u64> = Default::default();
        for record in &self.records {
            let timestamp = record.timestamp.clone();
            let r#type = record.typ.clone();
            let key_items = [
                record.filename.clone(),
                record.line.to_string(),
                record.name.clone(),
            ];
            let key = String::from(key_items.join("-"));
            if !frame_cache.contains_key::<String>(&key) {
                frame_cache.insert(key.clone(), frames.len() as u64);
                let frame = Frame {
                    name: record.name.clone(),
                    file: record.filename.clone(),
                    line: record.line,
                    col: 1,
                };
                frames.push(frame);
            }
            match frame_cache.get(&key) {
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
        let result = to_string(&events)?;
        Ok(result)
    }
    fn export_to_json(&mut self, filename: &str) -> PyResult<String> {
        let result = self._make_speed_scope_dict();

        // let result = match result {
        //     Ok(file) => file,
        //     Err(error) => error.to_string(),
        // };
        Ok(result)
    }
}

#[pymodule]
fn speedscope(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add("CustomError", py.get_type::<CustomError>())?;
    m.add_class::<SpRecorder>()?;
    Ok(())
}
