use pyo3::exceptions::{OSError, RuntimeError};
use pyo3::prelude::*;
use pyo3::{PyErr, PyResult};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::collections::HashMap;
use std::fmt;
use std::result::Result;

struct Record {
    timestamp: u64,
    typ: String,
    filename: String,
    line: u64,
    name: String,
}

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
struct SpRecorder {
    records: Vec<Record>,
}

pub fn make_error<E: fmt::Display + Sized>(e: E) -> PyErr {
    PyErr::new::<RuntimeError, _>(format!("{}", e))
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
    fn _make_speed_scope_dict(&mut self) -> Result<String, PyErr> {
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
        let speed_scope_struct = SpeedScopeStruct {
            schema: "https://www.speedscope.app/file-format-schema.json".to_string(),
            profiles: vec![Profile {
                r#type: "evented".to_string(),
                name: "python".to_string(),
                unit: "nanoseconds".to_string(),
                start_value: "".to_string(),
                end_value: "".to_string(),
                events: events,
            }],
            shared: ShareFrames { frames },
            active_profile_index: 0,
            exporter: "pyspeedscope".to_string(),
            name: "profile for python script".to_string(),
        };
        let result = to_string(&speed_scope_struct).map_err(make_error)?;
        Ok(result)
    }

    fn export_to_json(&mut self, filename: &str) -> PyResult<String> {
        let result = self._make_speed_scope_dict()?;
        Ok(result)
    }
}

#[pymodule]
fn speedscope(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<SpRecorder>()?;
    Ok(())
}
