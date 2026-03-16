#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct IntAttribute {
    pub name: String,
    pub value: i64,
}

impl IntAttribute {
    pub fn new(name: String, value: i64) -> Self {
        Self { name, value }
    }
}
