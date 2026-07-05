use crate::Error;
use ecitygml_core::model::relief::TinProperty;
use egml::io::primitives::GmlTriangulatedSurface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTinProperty {
    #[serde(
        rename(serialize = "@xlink:href", deserialize = "@href"),
        skip_serializing_if = "Option::is_none"
    )]
    pub href: Option<String>,

    #[serde(
        rename(
            serialize = "gml:TriangulatedSurface",
            deserialize = "TriangulatedSurface"
        ),
        skip_serializing_if = "Option::is_none"
    )]
    pub object: Option<GmlTriangulatedSurface>,
}

impl TryFrom<GmlTinProperty> for TinProperty {
    type Error = Error;

    fn try_from(item: GmlTinProperty) -> Result<Self, Self::Error> {
        Ok(Self {
            href: item.href,
            object: item.object.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<&TinProperty> for GmlTinProperty {
    fn from(item: &TinProperty) -> Self {
        Self {
            href: item.href.clone(),
            object: item.object.as_ref().map(|x| x.into()),
        }
    }
}
