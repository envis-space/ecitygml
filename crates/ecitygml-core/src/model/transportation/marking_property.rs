use crate::model::common::arena::{ArenaProperties, HasArenaProperties, HasArenaPropertiesMut};
use crate::model::transportation::Marking;
use egml::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use egml::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct MarkingProperty {
    object: Option<Marking>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
    arena: ArenaProperties,
}

impl MarkingProperty {
    pub fn new(
        object: Option<Marking>,
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

    pub fn from_object(object: Marking) -> Self {
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

    pub fn object(&self) -> Option<&Marking> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut Marking> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<Marking> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: Marking) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<Marking>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for MarkingProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for MarkingProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for MarkingProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for MarkingProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}

impl HasArenaProperties for MarkingProperty {
    fn arena_properties(&self) -> &ArenaProperties {
        &self.arena
    }
}

impl HasArenaPropertiesMut for MarkingProperty {
    fn arena_properties_mut(&mut self) -> &mut ArenaProperties {
        &mut self.arena
    }
}
