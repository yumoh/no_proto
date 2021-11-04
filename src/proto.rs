
use no_proto::NP_Factory;
use no_proto::buffer::NP_Buffer;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass]
pub struct Proto {
    factory: NP_Factory,
}

#[pymethods]
impl Proto {
    /// 使用es6语法定义数据结构
    /// struct({ fields: {
    ///    name: string(),
    ///    age: u16({ default: 0 }),
    ///    tags: list({ of: string() })
    /// }})
    #[staticmethod]
    pub fn from_factory_es6(schema: String) -> Self {
        let factory = NP_Factory::new(schema).expect("error schema");
        Self {
            factory
        }
    }
    /// 从导出的结构定义数据（bytes）中恢复数据结构定义
    #[staticmethod]
    pub fn from_factory_bytes(schema: &[u8]) -> Self {
        let factory = NP_Factory::new_bytes(schema).expect("error schema");
        Self {
            factory
        }
    }
    /// 导出结构定义数据（bytes）
    fn export_schema_bytes<'py>(&self,py:Python<'py>) -> &'py PyBytes {
        let b = self.factory.export_schema_bytes();
        PyBytes::new(py,b).into()
    }
    /// 创建空的 obj
    pub fn empty(&self) -> Buffer {
        let buffer = self.factory.new_buffer(None);
        Buffer {
            buffer
        }
    }
    /// 打开obj
    pub fn open(&self,bytes:&[u8]) -> Buffer {
        let buffer = self.factory.open_buffer(bytes.to_owned());
        Buffer {
            buffer
        }
    }
}

#[pyclass]
pub struct Buffer {
    buffer: NP_Buffer,
}

#[inline]
fn pad_value_json(json_value:&str) -> String {
    format!("{}\"value\":{}{}","{",json_value,"}")
}
#[inline]
fn tripe_value_json(js_value:String) -> String {
    js_value[9..js_value.len()-1].to_string()
}

#[pymethods]
impl Buffer {
    /// 结束编辑，关闭缓存，返回 obj -> bytes
    fn finish<'py>(&self,py:Python<'py>) -> &'py PyBytes {
        let b = self.buffer.clone();
        let bs = b.finish().bytes();
        PyBytes::new(py,&bs[..]).into()
    }
    /// 从 json 字符串中导入obj
    #[inline]
    fn from_json(&mut self,json_value:&str) -> bool {
        self.set_with_json(None, json_value)
    }
    /// 导出成json字符串
    #[inline]
    fn to_json(&mut self) -> String {
        self.json_encode(None)
        
    }
    /// 使用json字符串设置值
    fn set_with_json(&mut self,path:Option<Vec<&str>>,json_value:&str) -> bool {
        let p = path.unwrap_or(Vec::new());
        let js_value = pad_value_json(json_value);
        let result = self.buffer.set_with_json(&p,js_value).expect("error foramt");
        result
    }
    /// 获取指定的路径下的json字符串
    fn json_encode(&mut self,path:Option<Vec<&str>>) -> String {
        // let b = self.buffer.clone();
        let p = path.unwrap_or(Vec::new());
        let s = self.buffer.json_encode(&p).expect("error format!");
        tripe_value_json(s.stringify())
    }
    /// 获取str
    fn get_string(&self,path:Vec<&str>) -> Option<&str> {
        let result = self.buffer.get::<&str>(&path).unwrap();
        result
    }
    /// 设置str
    fn set_string(&mut self,path:Vec<&str>,value:&str) -> bool {
        self.buffer.set(&path, value).unwrap()
    }
    /// 设置（push）list[str]
    fn push_string(&mut self,path:Vec<&str>,value:&str) -> Option<u16> {
        let result =self.buffer.list_push(&path, value).unwrap();
        result
    }
    /// 获取int
    fn get_int(&self,path:Vec<&str>) -> Option<i64> {
        let result = self.buffer.get::<i64>(&path).unwrap();
        result
    }
    /// 设置int
    fn set_int(&mut self,path:Vec<&str>,value:i64) -> bool {
        self.buffer.set(&path, value).unwrap()
    }
    /// list[int]
    fn push_int(&mut self,path:Vec<&str>,value:i64) -> Option<u16> {
        let result =self.buffer.list_push(&path, value).unwrap();
        result
    }
    /// 获取float
    fn get_float(&self,path:Vec<&str>) -> Option<f64> {
        let result = self.buffer.get::<f64>(&path).unwrap();
        result
    }
    /// 设置float
    fn set_float(&mut self,path:Vec<&str>,value:f64) -> bool {
        self.buffer.set(&path, value).unwrap()
    }
    /// list[float]
    fn push_float(&mut self,path:Vec<&str>,value:f64) -> Option<u16> {
        let result =self.buffer.list_push(&path, value).unwrap();
        result
    }
}