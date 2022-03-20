use bytes::Bytes;
use rdr_zeromq::prelude::*;
use pyo3::prelude::*;
use rdr_core::prelude::encoded_img::EncodedImg;
use rdr_core::prelude::*;
// use pyo3_asyncio::tokio::future_into_py;

#[pyclass]
struct EncodedImgServer(rdr_zeromq::server::EncodedImgServer);

// #[pymethods]
// impl EncodedImgServer {
//     #[new]
//     fn new<'p>(py: Python<'p>, endpoint: &str) -> PyResult<&'p PyAny> {
//         let endpoint = endpoint.to_string();
//         pyo3_asyncio::tokio::future_into_py(py, async move {
//             Ok(EncodedImgServer(rdr_zeromq::server::EncodedImgServer::new(&endpoint).await))
//         })
//     }
// }

#[pyfunction]
fn new_img_server<'p>(py: Python<'p>, endpoint: &str) -> PyResult<&'p PyAny> {
    let endpoint = endpoint.to_string();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(EncodedImgServer(rdr_zeromq::server::EncodedImgServer::new(&endpoint).await))
    })
}

use pyo3::exceptions::PyOSError;
#[pyfunction]
fn img_server_send<'p>(py: Python<'p>, endpoint: Py<EncodedImgServer>) -> PyResult<&'p PyAny> {
    // let endpoint = endpoint.to_string();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let data = EncodedImg::new();
        data.timestamp = MessageField::some(Timestamp::now());
        data.data = Bytes::from("hello world");
        let x = endpoint.borrow_mut(py);
        x.0.send(&data).await.map_err(|e| PyOSError::new_err(e.to_string()))
    })
}

#[pymodule]
fn rdr_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new_img_server, m)?)?;
    m.add_class::<EncodedImgServer>()?;
    Ok(())
}