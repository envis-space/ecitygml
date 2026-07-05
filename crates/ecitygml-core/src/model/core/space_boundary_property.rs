use crate::model::core::space_boundary_kind::SpaceBoundaryKind;

#[derive(Debug, Clone, PartialEq)]
pub struct SpaceBoundaryProperty {
    pub object: Option<SpaceBoundaryKind>,
    pub href: Option<String>,
}

impl SpaceBoundaryProperty {
    pub fn new(object: SpaceBoundaryKind) -> Self {
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
