use crate::model::core::PointCloudKind;

#[derive(Debug, Clone, PartialEq)]
pub struct PointCloudProperty {
    pub object: Option<PointCloudKind>,
    pub href: Option<String>,
}

impl PointCloudProperty {
    pub fn new(object: PointCloudKind) -> Self {
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
