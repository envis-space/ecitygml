#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DoubleAttribute {
    pub name: String,
    pub value: f64,
}

impl DoubleAttribute {
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }
}
