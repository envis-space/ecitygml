use crate::model::building::BuildingPart;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingPartProperty {
    pub object: Option<BuildingPart>,
    pub href: Option<String>,
}

impl BuildingPartProperty {
    pub fn new(object: BuildingPart) -> Self {
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
