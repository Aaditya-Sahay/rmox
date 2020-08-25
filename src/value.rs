pub enum ValueType {
    Bool(bool),
    Number(f64),
    Nil
}

pub struct Value {
    pub value: ValueType
}

impl Value {
    pub fn new() -> Self {
        Self { value: ValueType::Nil }
    }
    pub fn from_bool(value: bool) -> Self {
        Self { value: ValueType::Bool(value) }
    }
    pub fn from_float(value: f64) -> Self {
        Self { value: ValueType::Number(value) }
    }
    pub fn as_bool(&self) -> Result<bool, &str> {
        if let ValueType::Bool(b) = self.value {
            return Ok(b);
        }
        else {
            return Err("something went wrong")
        }
    }

    pub fn as_float(&self) -> Result<f64, &str> {
        if let ValueType::Number(n) = self.value {
            return Ok(n);
        }
        else {
            return Err("something went wrong")
        }
    }

    pub fn print(&self) {
        match self.value {
            ValueType::Number(n) =>    println!("{}", n),
            ValueType::Bool(n) =>    println!("{}", n),
            ValueType::Nil =>    println!("nil")
        }
     
    }
}