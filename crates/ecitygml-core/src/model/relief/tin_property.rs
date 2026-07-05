use egml::model::geometry::primitives::TriangulatedSurface;

#[derive(Debug, Clone, PartialEq)]
pub struct TinProperty {
    pub object: Option<TriangulatedSurface>,
    pub href: Option<String>,
}

impl TinProperty {
    pub fn new(object: TriangulatedSurface) -> Self {
        Self {
            object: Some(object),
            href: None,
        }
    }

    pub fn new_href(href: impl Into<String>) -> Self {
        Self {
            object: None,
            href: Some(href.into()),
        }
    }
}
