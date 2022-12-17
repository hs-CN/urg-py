use pyo3::prelude::*;
use urg_rust;

#[pyclass]
#[derive(Debug)]
struct UrgVersionInfo {
    #[pyo3(get)]
    vendor_info: String,
    #[pyo3(get)]
    product_info: String,
    #[pyo3(get)]
    firmware_version: String,
    #[pyo3(get)]
    protocol_version: String,
    #[pyo3(get)]
    serial_number: String,
}

#[pymethods]
impl UrgVersionInfo {
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

impl TryFrom<urg_rust::UrgVersionInfo> for UrgVersionInfo {
    type Error = bstr::FromUtf8Error;

    fn try_from(value: urg_rust::UrgVersionInfo) -> Result<Self, Self::Error> {
        let urg_rust::UrgVersionInfo {
            vendor_info,
            product_info,
            firmware_version,
            protocol_version,
            serial_number,
        } = value;
        Ok(Self {
            vendor_info: vendor_info.try_into()?,
            product_info: product_info.try_into()?,
            firmware_version: firmware_version.try_into()?,
            protocol_version: protocol_version.try_into()?,
            serial_number: serial_number.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug)]
struct UrgSensorParams {
    #[pyo3(get)]
    sensor_model: String,
    #[pyo3(get)]
    min_distance_mm: u32,
    #[pyo3(get)]
    max_distance_mm: u32,
    #[pyo3(get)]
    angular_resolution_deg: f32,
    #[pyo3(get)]
    start_step: u32,
    #[pyo3(get)]
    end_step: u32,
    #[pyo3(get)]
    front_dir_step: u32,
    #[pyo3(get)]
    std_scan_speed_rpm: u32,
}

#[pymethods]
impl UrgSensorParams {
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

impl TryFrom<urg_rust::UrgSensorParams> for UrgSensorParams {
    type Error = bstr::FromUtf8Error;

    fn try_from(value: urg_rust::UrgSensorParams) -> Result<Self, Self::Error> {
        let urg_rust::UrgSensorParams {
            sensor_model,
            min_distance_mm,
            max_distance_mm,
            angular_resolution_deg,
            start_step,
            end_step,
            front_dir_step,
            std_scan_speed_rpm,
        } = value;
        Ok(Self {
            sensor_model: sensor_model.try_into()?,
            min_distance_mm,
            max_distance_mm,
            angular_resolution_deg,
            start_step,
            end_step,
            front_dir_step,
            std_scan_speed_rpm,
        })
    }
}

#[pyclass]
#[derive(Debug)]
struct UrgStatusInfo {
    #[pyo3(get)]
    sensor_model: String,
    #[pyo3(get)]
    laser_status: String,
    #[pyo3(get)]
    scanning_speed_rpm: u32,
    #[pyo3(get)]
    measurement_mode: String,
    #[pyo3(get)]
    communication_speed: String,
    #[pyo3(get)]
    time_stamp: u32,
    #[pyo3(get)]
    sensor_status: String,
}

#[pymethods]
impl UrgStatusInfo {
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

impl TryFrom<urg_rust::UrgStatusInfo> for UrgStatusInfo {
    type Error = bstr::FromUtf8Error;

    fn try_from(value: urg_rust::UrgStatusInfo) -> Result<Self, Self::Error> {
        let urg_rust::UrgStatusInfo {
            sensor_model,
            laser_status,
            scanning_speed_rpm,
            measurement_mode,
            communication_speed,
            time_stamp,
            sensor_status,
        } = value;
        Ok(Self {
            sensor_model: sensor_model.try_into()?,
            laser_status: laser_status.try_into()?,
            scanning_speed_rpm,
            measurement_mode: measurement_mode.try_into()?,
            communication_speed: communication_speed.try_into()?,
            time_stamp,
            sensor_status: sensor_status.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug)]
struct UrgPayload {
    #[pyo3(get)]
    time_stamp: u32,
    #[pyo3(get)]
    distance: Vec<u32>,
    #[pyo3(get)]
    intensity: Vec<u32>,
}

#[pymethods]
impl UrgPayload {
    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<urg_rust::UrgPayload> for UrgPayload {
    fn from(value: urg_rust::UrgPayload) -> Self {
        Self {
            time_stamp: value.time_stamp,
            distance: value.distance,
            intensity: value.intensity,
        }
    }
}

#[pyclass]
struct UrgPayloadIterator(urg_rust::UrgPayloadIterator);

#[pymethods]
impl UrgPayloadIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<UrgPayload> {
        slf.0
            .next()
            .and_then(|payload| Some(payload.unwrap().into()))
    }
}

#[pyclass]
struct Urg(urg_rust::Urg);

#[pymethods]
impl Urg {
    #[new]
    fn open(ip: &str, port: u16) -> PyResult<Self> {
        Ok(Self(urg_rust::Urg::open(ip.parse()?, port)?))
    }

    fn get_version_info(&self) -> PyResult<UrgVersionInfo> {
        self.0
            .get_version_info()?
            .try_into()
            .map_err(|err| pyo3::exceptions::PyUnicodeDecodeError::new_err(format!("{}", err)))
    }

    fn get_sensor_params(&self) -> PyResult<UrgSensorParams> {
        self.0
            .get_sensor_params()?
            .try_into()
            .map_err(|err| pyo3::exceptions::PyUnicodeDecodeError::new_err(format!("{}", err)))
    }

    fn get_status_info(&self) -> PyResult<UrgStatusInfo> {
        self.0
            .get_status_info()?
            .try_into()
            .map_err(|err| pyo3::exceptions::PyUnicodeDecodeError::new_err(format!("{}", err)))
    }

    fn start_capture(&mut self) -> PyResult<()> {
        Ok(self.0.start_capture()?)
    }

    fn stop_capture(&mut self) -> PyResult<()> {
        Ok(self.0.stop_capture()?)
    }

    fn get_distance(
        &self,
        start_step: u32,
        end_step: u32,
        cluster_count: u32,
    ) -> PyResult<UrgPayload> {
        Ok(self
            .0
            .get_distance(start_step, end_step, cluster_count)?
            .into())
    }

    fn get_distance_intensity(
        &self,
        start_step: u32,
        end_step: u32,
        cluster_count: u32,
    ) -> PyResult<UrgPayload> {
        Ok(self
            .0
            .get_distance_intensity(start_step, end_step, cluster_count)?
            .into())
    }

    fn get_distance_multi(
        &self,
        start_step: u32,
        end_step: u32,
        cluster_count: u32,
        scan_skip_count: u32,
        num_of_scan: u32,
    ) -> PyResult<UrgPayloadIterator> {
        Ok(UrgPayloadIterator(self.0.get_distance_multi(
            start_step,
            end_step,
            cluster_count,
            scan_skip_count,
            num_of_scan,
        )?))
    }

    fn get_distance_intensity_multi(
        &self,
        start_step: u32,
        end_step: u32,
        cluster_count: u32,
        scan_skip_count: u32,
        num_of_scan: u32,
    ) -> PyResult<UrgPayloadIterator> {
        Ok(UrgPayloadIterator(self.0.get_distance_intensity_multi(
            start_step,
            end_step,
            cluster_count,
            scan_skip_count,
            num_of_scan,
        )?))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn urg_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Urg>().unwrap();
    m.add_class::<UrgPayload>().unwrap();
    m.add_class::<UrgPayloadIterator>().unwrap();
    Ok(())
}
