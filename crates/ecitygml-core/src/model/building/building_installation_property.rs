use crate::model::building::BuildingInstallation;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingInstallationProperty {
    pub object: Option<BuildingInstallation>,
    pub href: Option<String>,
}

impl BuildingInstallationProperty {
    pub fn new(object: BuildingInstallation) -> Self {
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
