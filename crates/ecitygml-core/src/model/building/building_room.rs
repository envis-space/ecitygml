use crate::impl_abstract_unoccupied_space_traits;
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractFeatureMut, AsAbstractSpace, AsAbstractUnoccupiedSpace,
    AsAbstractUnoccupiedSpaceMut, CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingRoom {
    pub abstract_unoccupied_space: AbstractUnoccupiedSpace,
    pub(crate) class: Option<Code>,
    pub(crate) functions: Vec<Code>,
    pub(crate) usages: Vec<Code>,
}

impl BuildingRoom {
    pub fn new(abstract_unoccupied_space: AbstractUnoccupiedSpace) -> Self {
        BuildingRoom {
            abstract_unoccupied_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
        }
    }

    pub fn class(&self) -> &Option<Code> {
        &self.class
    }

    pub fn set_class(&mut self, class: Option<Code>) {
        self.class = class;
    }

    pub fn functions(&self) -> &Vec<Code> {
        &self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<Code>) {
        self.functions = functions;
    }

    pub fn usages(&self) -> &Vec<Code> {
        &self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<Code>) {
        self.usages = usages;
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::BuildingRoom(self))
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        /*self.wall_surface
        .iter_mut()
        .for_each(|x| x.refresh_bounded_by_recursive());*/

        let own_envelope = self.compute_envelope();
        let envelopes: Vec<Envelope> = own_envelope
            .as_ref()
            .into_iter()
            //.chain(self.wall_surface.iter().filter_map(|x| x.bounded_by()))
            .cloned()
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn apply_transform_recursive(&mut self, _m: &Isometry3<f64>) {
        // AsAbstractUnoccupiedSpace::apply_transform(&mut self.abstract_unoccupied_space, m);

        /*self.wall_surface
        .iter_mut()
        .for_each(|x| x.apply_transform(m));*/
    }
}

impl AsAbstractUnoccupiedSpace for BuildingRoom {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        &self.abstract_unoccupied_space
    }
}

impl AsAbstractUnoccupiedSpaceMut for BuildingRoom {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        &mut self.abstract_unoccupied_space
    }
}

impl_abstract_unoccupied_space_traits!(BuildingRoom);

impl From<BuildingRoom> for CityObjectKind {
    fn from(item: BuildingRoom) -> Self {
        CityObjectKind::BuildingRoom(item)
    }
}

impl Visitable for BuildingRoom {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_building_room(self);
    }
}
