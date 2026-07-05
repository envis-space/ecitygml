use crate::model::building::BuildingConstructiveElement;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingConstructiveElementProperty {
    pub object: Option<BuildingConstructiveElement>,
    pub href: Option<String>,
}

impl BuildingConstructiveElementProperty {
    pub fn new(object: BuildingConstructiveElement) -> Self {
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
