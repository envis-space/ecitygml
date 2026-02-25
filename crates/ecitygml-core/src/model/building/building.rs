use crate::model::building::building_constructive_element::BuildingConstructiveElement;
use crate::model::building::{AbstractBuilding, AsAbstractBuilding, AsAbstractBuildingMut};
use crate::model::construction::{GroundSurface, RoofSurface, WallSurface};
use crate::model::core::{
    AsAbstractFeature, AsAbstractFeatureMut, AsAbstractOccupiedSpace, AsAbstractOccupiedSpaceMut,
    AsAbstractSpaceMut, AsAbstractThematicSurfaceMut, CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Building {
    pub abstract_building: AbstractBuilding,
    pub wall_surface: Vec<WallSurface>,
    pub roof_surface: Vec<RoofSurface>,
    pub ground_surface: Vec<GroundSurface>,
    pub building_constructive_element: Vec<BuildingConstructiveElement>,
}

impl Building {
    pub fn new(abstract_building: AbstractBuilding) -> Self {
        Self {
            abstract_building,
            wall_surface: Vec::new(),
            roof_surface: Vec::new(),
            ground_surface: Vec::new(),
            building_constructive_element: Vec::new(),
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::Building(self))
            .chain(self.wall_surface.iter().flat_map(|x| x.iter_city_object()))
            .chain(self.roof_surface.iter().flat_map(|x| x.iter_city_object()))
            .chain(
                self.ground_surface
                    .iter()
                    .flat_map(|x| x.iter_city_object()),
            )
            .chain(
                self.building_constructive_element
                    .iter()
                    .flat_map(|x| x.iter_city_object()),
            )
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.wall_surface
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by_recursive());
        self.roof_surface
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by());
        self.ground_surface
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by());
        self.building_constructive_element
            .iter_mut()
            .for_each(AsAbstractOccupiedSpaceMut::refresh_bounded_by);

        let own_envelope = self.compute_envelope();
        let envelopes: Vec<&Envelope> = own_envelope
            .as_ref()
            .into_iter()
            .chain(self.wall_surface.iter().filter_map(|x| x.bounded_by()))
            .chain(self.roof_surface.iter().filter_map(|x| x.bounded_by()))
            .chain(self.ground_surface.iter().filter_map(|x| x.bounded_by()))
            .chain(
                self.building_constructive_element
                    .iter()
                    .filter_map(|x| x.bounded_by()),
            )
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn apply_transform_recursive(&mut self, m: &Isometry3<f64>) {
        AsAbstractBuildingMut::apply_transform(&mut self.abstract_building, m);

        self.wall_surface
            .iter_mut()
            .for_each(|x| x.apply_transform(m));
        self.roof_surface
            .iter_mut()
            .for_each(|x| x.apply_transform(m));
        self.ground_surface
            .iter_mut()
            .for_each(|x| x.apply_transform(m));
        self.building_constructive_element
            .iter_mut()
            .for_each(|x| AsAbstractOccupiedSpaceMut::apply_transform(x, m));
    }
}

impl AsAbstractBuilding for Building {
    fn abstract_building(&self) -> &AbstractBuilding {
        &self.abstract_building
    }
}

impl AsAbstractBuildingMut for Building {
    fn abstract_building_mut(&mut self) -> &mut AbstractBuilding {
        &mut self.abstract_building
    }
}

crate::impl_abstract_building_traits!(Building);

impl From<Building> for CityObjectKind {
    fn from(item: Building) -> Self {
        CityObjectKind::Building(item)
    }
}

impl Visitable for Building {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_building(self);
        self.wall_surface.iter().for_each(|x| x.accept(visitor));
        self.roof_surface.iter().for_each(|x| x.accept(visitor));
        self.ground_surface.iter().for_each(|x| x.accept(visitor));
        self.building_constructive_element
            .iter()
            .for_each(|x| x.accept(visitor));
    }
}
