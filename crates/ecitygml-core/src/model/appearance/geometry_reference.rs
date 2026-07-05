#[derive(Debug, Clone, PartialEq)]
pub struct GeometryReference {
    pub href: String,
}

impl GeometryReference {
    pub fn new(href: String) -> Self {
        Self { href }
    }
}
