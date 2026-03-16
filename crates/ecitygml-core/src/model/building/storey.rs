use crate::impl_abstract_building_subdivision_traits;
use crate::model::building::{
    AbstractBuildingSubdivision, AsAbstractBuildingSubdivision, AsAbstractBuildingSubdivisionMut,
};
use crate::model::core::{AsAbstractFeatureMut, AsAbstractSpace, CityObjectKind, CityObjectRef};
use crate::operations::{Visitable, Visitor};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Storey {
    pub abstract_building_subdivision: AbstractBuildingSubdivision,
}

impl Storey {
    pub fn new(abstract_building_subdivision: AbstractBuildingSubdivision) -> Self {
        Self {
            abstract_building_subdivision,
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::Storey(self))
    }

    pub fn refresh_bounded_by(&mut self) {
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

impl AsAbstractBuildingSubdivision for Storey {
    fn abstract_building_subdivision(&self) -> &AbstractBuildingSubdivision {
        &self.abstract_building_subdivision
    }
}

impl AsAbstractBuildingSubdivisionMut for Storey {
    fn abstract_building_subdivision_mut(&mut self) -> &mut AbstractBuildingSubdivision {
        &mut self.abstract_building_subdivision
    }
}

impl_abstract_building_subdivision_traits!(Storey);

impl From<Storey> for CityObjectKind {
    fn from(item: Storey) -> Self {
        CityObjectKind::Storey(item)
    }
}

impl Visitable for Storey {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_storey(self);
    }
}
