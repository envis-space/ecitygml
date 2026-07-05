use egml::model::geometry::Envelope;
use egml::model::geometry::primitives::PointProperty;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct ImplicitGeometry {
    pub reference_point: PointProperty,
}

impl ImplicitGeometry {
    pub fn new(reference_point: PointProperty) -> Self {
        Self { reference_point }
    }
}

impl ImplicitGeometry {
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(point) = self.reference_point.object.as_mut() {
            point.apply_transform(m);
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        // TODO: compute envelope containing referenced geometry
        if let Some(point) = self.reference_point.object.as_ref() {
            return Some(point.compute_envelope());
        }

        None
    }
}
