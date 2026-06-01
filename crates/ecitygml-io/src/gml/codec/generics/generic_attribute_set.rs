use crate::Error;
use crate::gml::codec::generics::GmlGenericAttributeProperty;
use ecitygml_core::model::generics::{GenericAttributeKind, GenericAttributeSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GmlGenericAttributeSet {
    #[serde(rename(serialize = "gen:name", deserialize = "name"))]
    pub name: String,
    #[serde(
        default,
        rename(serialize = "gen:genericAttribute", deserialize = "genericAttribute")
    )]
    pub generic_attribute: Vec<GmlGenericAttributeProperty>,
}

impl TryFrom<GmlGenericAttributeSet> for GenericAttributeSet {
    type Error = Error;

    fn try_from(item: GmlGenericAttributeSet) -> Result<Self, Self::Error> {
        if item.name.is_empty() {
            tracing::debug!("AttributeSet with empty string as name");
            // return Err(AttributeWithoutName("generic attribute set".to_string()));
        }

        let generic_attributes: Vec<GenericAttributeKind> = item
            .generic_attribute
            .into_iter()
            .map(|x| x.content.try_into())
            .collect::<Result<Vec<GenericAttributeKind>, Error>>()?;

        Ok(Self {
            name: item.name,
            generic_attributes,
        })
    }
}

impl From<&GenericAttributeSet> for GmlGenericAttributeSet {
    fn from(attr: &GenericAttributeSet) -> Self {
        Self {
            name: attr.name.clone(),
            generic_attribute: attr
                .generic_attributes
                .iter()
                .map(|x| GmlGenericAttributeProperty { content: x.into() })
                .collect(),
        }
    }
}
