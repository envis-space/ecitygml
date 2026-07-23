use crate::model::common::arena::{ArenaProperties, HasArenaProperties, HasArenaPropertiesMut};
use crate::model::construction::AbstractFillingSurfaceKind;
use egml::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use egml::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractFillingSurfaceProperty {
    object: Option<AbstractFillingSurfaceKind>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
    arena: ArenaProperties,
}

impl AbstractFillingSurfaceProperty {
    pub fn new(
        object: Option<AbstractFillingSurfaceKind>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
            arena: ArenaProperties::default(),
        }
    }

    pub fn from_object(object: AbstractFillingSurfaceKind) -> Self {
        Self {
            object: Some(object),
            association: AssociationAttributes::default(),
            ownership: OwnershipAttributes::default(),
            arena: ArenaProperties::default(),
        }
    }

    pub fn from_href(href: HRef) -> Self {
        Self {
            object: None,
            association: AssociationAttributes::new_href(href),
            ownership: OwnershipAttributes::default(),
            arena: ArenaProperties::default(),
        }
    }

    pub fn object(&self) -> Option<&AbstractFillingSurfaceKind> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut AbstractFillingSurfaceKind> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<AbstractFillingSurfaceKind> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: AbstractFillingSurfaceKind) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<AbstractFillingSurfaceKind>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for AbstractFillingSurfaceProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for AbstractFillingSurfaceProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for AbstractFillingSurfaceProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for AbstractFillingSurfaceProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}

impl HasArenaProperties for AbstractFillingSurfaceProperty {
    fn arena_properties(&self) -> &ArenaProperties {
        &self.arena
    }
}

impl HasArenaPropertiesMut for AbstractFillingSurfaceProperty {
    fn arena_properties_mut(&mut self) -> &mut ArenaProperties {
        &mut self.arena
    }
}
