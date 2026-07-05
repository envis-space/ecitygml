use crate::model::building::BuildingRoom;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingRoomProperty {
    pub object: Option<BuildingRoom>,
    pub href: Option<String>,
}

impl BuildingRoomProperty {
    pub fn new(object: BuildingRoom) -> Self {
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
