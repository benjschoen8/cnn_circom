use pyo3::prelude::*;
use pyo3::types::PyModule;
use crate::adapter::python_embedding::PythonEmbedding;

pub struct Pytorch;

impl Pytorch {
    pub fn new() -> Self {
        Self
    }
}

impl PythonEmbedding for Pytorch {
    fn train(&self, _dataset: &str) -> Result<String, String> {
        Python::with_gil(|py| {
            let sys = PyModule::import_bound(py, "sys").map_err(|e| e.to_string())?;
            let sys_path: &pyo3::types::PyList = sys.getattr("path").map_err(|e| e.to_string())?.extract().map_err(|e| e.to_string())?;
            sys_path.insert(0, "./src/framework/python").map_err(|e| e.to_string())?;

            let pytorch_module = PyModule::import_bound(py, "pytorch").map_err(|e| e.to_string())?;

            let trained_json: String = pytorch_module
                .getattr("train").map_err(|e| e.to_string())?
                .call0().map_err(|e| e.to_string())?
                .extract().map_err(|e| e.to_string())?;

            Ok(trained_json)
        })
    }
}

