use crate::model::appearance::SurfaceDataKind;

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceDataProperty {
    pub object: Option<SurfaceDataKind>,
    pub href: Option<String>,
}

impl SurfaceDataProperty {
    pub fn new(object: SurfaceDataKind) -> Self {
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
