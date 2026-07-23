use crate::model::core::ImplicitGeometry;
use egml::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use egml::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct ImplicitGeometryProperty {
    object: Option<ImplicitGeometry>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl ImplicitGeometryProperty {
    pub fn new(
        object: Option<ImplicitGeometry>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: ImplicitGeometry) -> Self {
        Self {
            object: Some(object),
            association: AssociationAttributes::default(),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn from_href(href: HRef) -> Self {
        Self {
            object: None,
            association: AssociationAttributes::new_href(href),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn object(&self) -> Option<&ImplicitGeometry> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut ImplicitGeometry> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<ImplicitGeometry> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: ImplicitGeometry) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<ImplicitGeometry>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for ImplicitGeometryProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for ImplicitGeometryProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for ImplicitGeometryProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for ImplicitGeometryProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
