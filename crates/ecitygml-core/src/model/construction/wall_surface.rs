use crate::model::construction::{
    AbstractConstructionSurface, AsAbstractConstructionSurface, AsAbstractConstructionSurfaceMut,
    DoorSurface, WindowSurface,
};
use crate::model::core::{
    AsAbstractFeature, AsAbstractFeatureMut, AsAbstractThematicSurface,
    AsAbstractThematicSurfaceMut, CityObjectKind, CityObjectRef,
};
use crate::operations::{Visitable, Visitor};
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct WallSurface {
    pub abstract_construction_surface: AbstractConstructionSurface,
    pub door_surface: Vec<DoorSurface>,
    pub window_surface: Vec<WindowSurface>,
}

impl WallSurface {
    pub fn new(abstract_construction_surface: AbstractConstructionSurface) -> Self {
        Self {
            abstract_construction_surface,
            door_surface: Vec::new(),
            window_surface: Vec::new(),
        }
    }

    pub fn iter_city_object<'a>(&'a self) -> impl Iterator<Item = CityObjectRef<'a>> + 'a {
        std::iter::once(CityObjectRef::WallSurface(self))
            .chain(self.door_surface.iter().flat_map(|x| x.iter_city_object()))
            .chain(
                self.window_surface
                    .iter()
                    .flat_map(|x| x.iter_city_object()),
            )
    }

    pub fn refresh_bounded_by_recursive(&mut self) {
        self.door_surface
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by());
        self.window_surface
            .iter_mut()
            .for_each(|x| x.refresh_bounded_by());

        let own_envelope = self.compute_envelope();
        let envelopes: Vec<Envelope> = own_envelope
            .as_ref()
            .into_iter()
            .chain(self.door_surface.iter().filter_map(|x| x.bounded_by()))
            .chain(self.window_surface.iter().filter_map(|x| x.bounded_by()))
            .cloned()
            .collect();

        self.set_bounded_by(Envelope::from_envelopes(&envelopes));
    }

    pub fn apply_transform_recursive(&mut self, m: &Isometry3<f64>) {
        self.abstract_construction_surface.apply_transform(m);

        self.door_surface
            .iter_mut()
            .for_each(|x| x.apply_transform(m));
        self.window_surface
            .iter_mut()
            .for_each(|x| x.apply_transform(m));
    }
}

impl AsAbstractConstructionSurface for WallSurface {
    fn abstract_construction_surface(&self) -> &AbstractConstructionSurface {
        &self.abstract_construction_surface
    }
}

impl AsAbstractConstructionSurfaceMut for WallSurface {
    fn abstract_construction_surface_mut(&mut self) -> &mut AbstractConstructionSurface {
        &mut self.abstract_construction_surface
    }
}

crate::impl_abstract_construction_surface_traits!(WallSurface);

impl From<WallSurface> for CityObjectKind {
    fn from(item: WallSurface) -> Self {
        CityObjectKind::WallSurface(item)
    }
}

impl Visitable for WallSurface {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_wall_surface(self);
        self.door_surface.iter().for_each(|x| x.accept(visitor));
        self.window_surface.iter().for_each(|x| x.accept(visitor));
    }
}
