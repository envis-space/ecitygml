use crate::model::transportation::TrafficSpace;

#[derive(Debug, Clone, PartialEq)]
pub struct TrafficSpaceProperty {
    pub object: Option<TrafficSpace>,
    pub href: Option<String>,
}

impl TrafficSpaceProperty {
    pub fn new(object: TrafficSpace) -> Self {
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
