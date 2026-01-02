use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use super::enums::{
    PyDuplicateKeyMode, PyElsetKeyMode, PyKeyMode, py_duplicate_key_mode, py_elset_key_mode, py_key_mode,
};
use crate::{
    ALL_KEYMODE_DMA, ALL_KEYMODE_NODUP, DLL_VERSION, DUPKEY_ACTUAL, DUPKEY_ZERO, ELSET_KEYMODE_DMA,
    ELSET_KEYMODE_NODUP, get_dll_info, get_duplicate_key_mode, get_elset_key_mode, get_key_mode,
    get_last_error_message, get_last_info_message, load_from_file, reset_key_mode, set_duplicate_key_mode,
    set_elset_key_mode, set_key_mode,
};

#[pyclass]
pub struct MainInterface {
    info: String,
}

#[pymethods]
impl MainInterface {
    #[new]
    #[pyo3(signature = (file_name=None))]
    fn new(file_name: Option<String>) -> PyResult<Self> {
        let info = get_dll_info();
        if !info.contains(DLL_VERSION) {
            return Err(PyRuntimeError::new_err(format!(
                "Expected DLL {} inconsistent with {}",
                DLL_VERSION, info
            )));
        }
        if let Some(file) = file_name {
            load_from_file(&file).map_err(PyRuntimeError::new_err)?;
        }
        Ok(MainInterface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    #[getter]
    fn last_error_message(&self) -> PyResult<String> {
        Ok(get_last_error_message())
    }

    #[getter]
    fn last_info_message(&self) -> PyResult<String> {
        Ok(get_last_info_message())
    }

    #[getter]
    fn key_mode(&self, py: Python<'_>) -> PyResult<PyObject> {
        let mode = get_key_mode().map_err(PyRuntimeError::new_err)?;
        py_key_mode(py, mode)
    }

    #[setter]
    fn set_key_mode(&self, mode: PyKeyMode) -> PyResult<()> {
        set_key_mode(mode.into()).map_err(PyRuntimeError::new_err)
    }

    fn reset_key_mode(&self) {
        reset_key_mode();
    }

    #[getter]
    fn elset_key_mode(&self, py: Python<'_>) -> PyResult<PyObject> {
        let mode = get_elset_key_mode().map_err(PyRuntimeError::new_err)?;
        py_elset_key_mode(py, mode)
    }

    #[setter]
    fn set_elset_key_mode(&self, mode: PyElsetKeyMode) -> PyResult<()> {
        set_elset_key_mode(mode.into()).map_err(PyRuntimeError::new_err)
    }

    #[getter]
    fn duplicate_key_mode(&self, py: Python<'_>) -> PyResult<PyObject> {
        let mode = get_duplicate_key_mode().map_err(PyRuntimeError::new_err)?;
        py_duplicate_key_mode(py, mode)
    }

    #[setter]
    fn set_duplicate_key_mode(&self, mode: PyDuplicateKeyMode) -> PyResult<()> {
        set_duplicate_key_mode(mode.into()).map_err(PyRuntimeError::new_err)
    }

    #[classattr]
    const DLL_VERSION: &'static str = DLL_VERSION;
}
pub fn register_main_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<MainInterface>()?;
    Ok(())
}
