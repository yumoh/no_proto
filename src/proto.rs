\use no_proto::NP_Factory;
use no_proto::buffer::NP_Buffer;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass]
pub struct Proto {
    factory: NP_Factory,
}

#[pymethods]
impl Proto {

    #[staticmethod]
    pub fn from_factory_es6(schema: String) -> Self {
        let factory = NP_Factory::new(schema).expect("error schema");
        Self {
            factory
        }
    }
    #[staticmethod]
    pub fn from_factory_bytes(schema: &[u8]) -> Self {
        let factory = NP_Factory::new_bytes(schema).expect("error schema");
        Self {
            factory
        }
    }

    fn export_schema_bytes<'py>(&self,py:Python<'py>) -> &'py PyBytes {
        let b = self.factory.export_schema_bytes();
        PyBytes::new(py,b).into()
    }

    pub fn empty(&self) -> Buffer {
        let buffer = self.factory.new_buffer(None);
        Buffer {
            buffer
        }
    }

    pub fn open(&self,bytes:Vec<u8>) -> Buffer {
        let buffer = self.factory.open_buffer(bytes);
        Buffer {
            buffer
        }
    }
}

#[pyclass]
pub struct Buffer {
    buffer: NP_Buffer,
}

#[pymethods]
impl Buffer {

    fn finish<'py>(&self,py:Python<'py>) -> &'py PyBytes {
        let b = self.buffer.clone();
        let bs = b.finish().bytes();
        PyBytes::new(py,&bs[..]).into()
    }

    #[inline]
    fn from_json(&mut self,json_value:String) -> bool {
        self.set_with_json(None, json_value)
    }

    #[inline]
    fn to_json(&mut self) -> String {
        self.json_encode(None)
    }

    fn set_with_json(&mut self,path:Option<Vec<&str>>,json_value:String) -> bool {
        let p = path.unwrap_or(Vec::new());
        let result = self.buffer.set_with_json(&p, json_value).expect("error foramt");
        result
    }

    fn json_encode(&mut self,path:Option<Vec<&str>>) -> String {
        // let b = self.buffer.clone();
        let p = path.unwrap_or(Vec::new());
        let s = self.buffer.json_encode(&p).expect("error format!");
        s.stringify()
    }

    fn get_string(&self,path:Vec<&str>) -> Option<&str> {
        let result = self.buffer.get::<&str>(&path).unwrap();
        result
    }

    fn set_string(&mut self,path:Vec<&str>,value:&str) -> bool {
        self.buffer.set(&path, value).unwrap()
    }

    fn push_string(&mut self,path:Vec<&str>,value:&str) -> Option<u16> {
        let result =self.buffer.list_push(&path, value).unwrap();
        result
    }

    fn get_int(&self,path:Vec<&str>) -> Option<i64> {
        let result = self.buffer.get::<i64>(&path).unwrap();
        result
    }

    fn set_int(&mut self,path:Vec<&str>,value:i64) -> bool {
        self.buffer.set(&path, value).unwrap()
    }

    fn push_int(&mut self,path:Vec<&str>,value:i64) -> Option<u16> {
        let result =self.buffer.list_push(&path, value).unwrap();
        result
    }

    fn get_float(&self,path:Vec<&str>) -> Option<f64> {
        let result = self.buffer.get::<f64>(&path).unwrap();
        result
    }

    fn set_float(&mut self,path:Vec<&str>,value:f64) -> bool {
        self.buffer.set(&path, value).unwrap()
    }

    fn push_float(&mut self,path:Vec<&str>,value:f64) -> Option<u16> {
        let result =self.buffer.list_push(&path, value).unwrap();
        result
    }

}