use egml::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use egml::model::geometry::primitives::TriangulatedSurface;
use egml::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct TinProperty {
    object: Option<TriangulatedSurface>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl TinProperty {
    pub fn new(
        object: Option<TriangulatedSurface>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: TriangulatedSurface) -> Self {
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

    pub fn object(&self) -> Option<&TriangulatedSurface> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut TriangulatedSurface> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<TriangulatedSurface> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: TriangulatedSurface) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<TriangulatedSurface>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for TinProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for TinProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for TinProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for TinProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
