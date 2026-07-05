use crate::model::building::BuildingSubdivisionKind;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingSubdivisionProperty {
    pub object: Option<BuildingSubdivisionKind>,
    pub href: Option<String>,
}

impl BuildingSubdivisionProperty {
    pub fn new(object: BuildingSubdivisionKind) -> Self {
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
