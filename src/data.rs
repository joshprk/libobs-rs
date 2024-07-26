use std::collections::HashMap;
use std::ffi::c_longlong;
use std::ffi::c_double;

use crate::ffi;
use crate::ffi::obs_data;
use crate::ObsString;

trait DataType {
    fn get(&self) -> Data;
}

impl DataType for c_longlong {
    fn get(&self) -> Data {
        Data::Int(*self)
    }
}

impl DataType for c_double {
    fn get(&self) -> Data {
        Data::Double(*self)
    }
}

impl DataType for ObsString {
    fn get(&self) -> Data {
        Data::Str(self.clone())
    }
}

impl DataType for bool {
    fn get(&self) -> Data {
        Data::Bool(*self)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Data {
    Int(i64),
    Double(f64),
    Str(ObsString),
    Bool(bool),
}

impl Data {
    pub fn is_int(self) -> i64 {
        match self {
            Self::Int(value)    => value,
            _                   => panic!(),
        }
    }

    pub fn is_double(self) -> f64 {
        match self {
            Self::Double(value) => value,
            _                   => panic!(),
        }
    }

    pub fn is_str(self) -> ObsString {
        match self {
            Self::Str(value)    => value,
            _                   => panic!(),
        }
    }

    pub fn is_bool(self) -> bool {
        match self {
            Self::Bool(value)   => value,
            _                   => panic!(),
        }
    }
}

pub struct ObsData<'a> {
    ptr: *mut obs_data,
    map: HashMap<ObsString, Box<dyn DataType + 'a>>,
}

impl<'a> ObsData<'a> {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::obs_data_create() };
        let map = HashMap::new();

        Self { ptr, map }
    }

    pub fn get(&mut self, key: impl Into<ObsString>) -> Option<Data> {
        let key = key.into();
        let Some(value) = self.map.get(&key) else {
            return None
        };

        Some(value.get())
    }

    pub fn int(&mut self, key: impl Into<ObsString>, value: impl Into<c_longlong>) -> &mut Self {
        let key = key.into();
        let value = value.into();

        unsafe { ffi::obs_data_set_int(self.ptr, key.as_ptr(), value) }
        self.map.insert(key, Box::new(value));
        self
    }

    pub fn str(&mut self, key: impl Into<ObsString>, value: impl Into<ObsString>) -> &mut Self {
        let key = key.into();
        let value = value.into();

        unsafe { ffi::obs_data_set_string(self.ptr, key.as_ptr(), value.as_ptr()) }
        self.map.insert(key, Box::new(value));
        self
    }

    pub fn bool(&mut self, key: impl Into<ObsString>, value: bool) -> &mut Self {
        let key = key.into();
        let value = value.into();

        unsafe { ffi::obs_data_set_bool(self.ptr, key.as_ptr(), value) }
        self.map.insert(key, Box::new(value));
        self
    }

    pub fn double(&mut self, key: impl Into<ObsString>, value: impl Into<f64>) -> &mut Self {
        let key = key.into();
        let value = value.into();

        unsafe { ffi::obs_data_set_double(self.ptr, key.as_ptr(), value) }
        self.map.insert(key, Box::new(value));
        self
    }

    pub fn erase(&mut self, key: impl Into<ObsString>) -> &mut Self {
        let key = key.into();
        unsafe { ffi::obs_data_erase(self.ptr, key.as_ptr()) }
        self.map.remove(&key);
        self
    }

    pub fn merge(&mut self, mut data: ObsData<'a>) -> &mut Self {
        unsafe { ffi::obs_data_apply(self.ptr, data.ptr) }
        self.map.extend(data.map.drain());
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        unsafe { ffi::obs_data_clear(self.ptr) }
        self.map.clear();
        self
    }

    pub fn shrink(&mut self) -> &mut Self {
        self.map.shrink_to_fit();
        self
    }
}

impl<'a> Drop for ObsData<'a> {
    fn drop(&mut self) {
        unsafe { ffi::obs_data_release(self.ptr) }
    }
}