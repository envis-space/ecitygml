use egml::model::geometry::primitives::Point;
use egml::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImplicitGeometry {
    pub reference_point: Point,
}

impl ImplicitGeometry {
    pub fn new(reference_point: Point) -> Self {
        Self { reference_point }
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        vec![&self.reference_point.pos()]
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.reference_point.apply_transform(m);
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        // TODO: compute envelope containing referenced geometry
        Some(self.reference_point.compute_envelope())
    }
}
