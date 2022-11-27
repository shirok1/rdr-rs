use bytes::Bytes;
use rdr_zeromq::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDateTime, PyTime};
use rdr_core::message::bag::*;

#[derive(Clone)]
#[pyclass]
pub enum Color {
    BLUE = 0,
    RED = 1,
    N = 2,
    P = 3,
}

#[derive(Clone)]
#[pyclass]
pub enum ArmorType {
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.GUARD)
    GUARD = 0,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.N1)
    N1 = 1,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.N2)
    N2 = 2,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.N3)
    N3 = 3,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.N4)
    N4 = 4,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.N5)
    N5 = 5,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.OUTPOST)
    OUTPOST = 6,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.BASE_SMALL)
    BASE_SMALL = 7,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.BASE_BIG)
    BASE_BIG = 8,
    // @@protoc_insertion_point(enum_value:rdr.Armor.Type.NONE)
    NONE = 9,
}

#[derive(Clone)]
#[pyclass]
pub struct Armor {
    #[pyo3(get)]
    pub position: (f32, f32),
    #[pyo3(get)]
    pub size: (f32, f32),
    #[pyo3(get)]
    pub confidence: f32,
    #[pyo3(get)]
    pub color: Color,
    #[pyo3(get)]
    pub armor_type: ArmorType,
}

#[derive(Clone)]
#[pyclass]
pub struct CarInfo {
    #[pyo3(get)]
    pub armor: Armor,
    #[pyo3(get)]
    pub car: Car,
}

#[derive(Clone)]
#[pyclass]
pub struct Car {
    #[pyo3(get)]
    pub position: (f32, f32),
    #[pyo3(get)]
    pub size: (f32, f32),
    #[pyo3(get)]
    pub confidence: f32,
    #[pyo3(get)]
    pub color: Color,
    #[pyo3(get)]
    pub car_type: CarType,
}

#[derive(Clone)]
#[pyclass]
pub enum CarType {
    // @@protoc_insertion_point(enum_value:rdr.Car.Type.CAR)
    CAR = 0,
    // @@protoc_insertion_point(enum_value:rdr.Car.Type.WATCHER)
    WATCHER = 1,
    // @@protoc_insertion_point(enum_value:rdr.Car.Type.BASE)
    BASE = 2,
}

#[derive(Clone)]
#[pyclass]
pub struct DetectedArmorBinding {
    #[pyo3(get, set)]
    timestamp: Py<PyDateTime>,
    #[pyo3(get)]
    armors: Vec<CarInfo>,
}

#[derive(Clone)]
#[pyclass]
pub struct DetectedArmorBagBinding {
    #[pyo3(get, set)]
    timestamp: Py<PyDateTime>,
    #[pyo3(get)]
    sequence: Vec<DetectedArmorBinding>,
}

#[pyfunction]
fn decode_detected_armor_bag(py: Python<'_>, bin: &[u8]) -> PyResult<DetectedArmorBagBinding> {
    // PyTime::
    DetectedArmorBag::parse_from_bytes(bin)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
        .map(|bag: DetectedArmorBag| {
            DetectedArmorBagBinding {
                timestamp: PyDateTime::from_timestamp(py, bag.timestamp.seconds as f64, None).unwrap().into(),
                sequence: bag.sequence.into_iter().map(|binding| {
                    DetectedArmorBinding {
                        timestamp: PyDateTime::from_timestamp(py, binding.timestamp.seconds as f64, None).unwrap().into(),
                        armors: binding.armors.into_iter().map(|car_info| {
                            CarInfo {
                                armor: Armor {
                                    position: (car_info.armor.x, car_info.armor.y),
                                    size: (car_info.armor.height, car_info.armor.width),
                                    confidence: car_info.armor.confidence,
                                    color: match car_info.armor.color { _ => Color::BLUE },
                                    armor_type: match car_info.armor.type_ { _ => ArmorType::GUARD },
                                },
                                car: Car {
                                    position: (car_info.car.x, car_info.car.y),
                                    size: (car_info.car.height, car_info.car.width),
                                    confidence: car_info.car.confidence,
                                    color: match car_info.car.color { _ => Color::BLUE },
                                    car_type: match car_info.car.type_ { _ => CarType::CAR },
                                },
                            }
                        }).collect(),
                    }
                }).collect(),
            }
        })
}

//noinspection SpellCheckingInspection
/// This module is implemented in Rust.
#[pymodule]
#[pyo3(name = "rdr")]
fn rdr(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_detected_armor_bag, m)?)?;
    Ok(())
}