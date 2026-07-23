use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::AbstractFeatureKindRef;
use crate::model::core::refs::AbstractFeatureKindRefMut;
use crate::model::transportation::values::{RoadClassValue, RoadFunctionValue, RoadUsageValue};
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
    IntersectionProperty, SectionProperty,
};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct Road {
    pub(crate) abstract_transportation_space: AbstractTransportationSpace,
    class: Option<RoadClassValue>,
    functions: Vec<RoadFunctionValue>,
    usages: Vec<RoadUsageValue>,
    sections: Vec<SectionProperty>,
    intersections: Vec<IntersectionProperty>,
}

impl Road {
    pub fn new(id: Id) -> Self {
        Self::from_abstract_transportation_space(AbstractTransportationSpace::new(id))
    }

    pub fn from_abstract_transportation_space(
        abstract_transportation_space: AbstractTransportationSpace,
    ) -> Self {
        Self {
            abstract_transportation_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            sections: Default::default(),
            intersections: Default::default(),
        }
    }

    pub fn class(&self) -> Option<&RoadClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: RoadClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<RoadClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[RoadFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [RoadFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<RoadFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: RoadFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(&mut self, functions: impl IntoIterator<Item = RoadFunctionValue>) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[RoadUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [RoadUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<RoadUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: RoadUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = RoadUsageValue>) {
        self.usages.extend(usages);
    }

    pub fn sections(&self) -> &[SectionProperty] {
        &self.sections
    }

    pub fn sections_mut(&mut self) -> &mut [SectionProperty] {
        &mut self.sections
    }

    pub fn set_sections(&mut self, sections: Vec<SectionProperty>) {
        self.sections = sections;
    }

    pub fn push_section(&mut self, section: SectionProperty) {
        self.sections.push(section);
    }

    pub fn extend_sections(&mut self, sections: impl IntoIterator<Item = SectionProperty>) {
        self.sections.extend(sections);
    }

    pub fn intersections(&self) -> &[IntersectionProperty] {
        &self.intersections
    }

    pub fn intersections_mut(&mut self) -> &mut [IntersectionProperty] {
        &mut self.intersections
    }

    pub fn set_intersections(&mut self, intersections: Vec<IntersectionProperty>) {
        self.intersections = intersections;
    }

    pub fn push_intersection(&mut self, intersection: IntersectionProperty) {
        self.intersections.push(intersection);
    }

    pub fn extend_intersections(
        &mut self,
        intersections: impl IntoIterator<Item = IntersectionProperty>,
    ) {
        self.intersections.extend(intersections);
    }
}

impl AsAbstractTransportationSpace for Road {
    fn abstract_transportation_space(&self) -> &AbstractTransportationSpace {
        &self.abstract_transportation_space
    }
}

impl AsAbstractTransportationSpaceMut for Road {
    fn abstract_transportation_space_mut(&mut self) -> &mut AbstractTransportationSpace {
        &mut self.abstract_transportation_space
    }
}

crate::impl_abstract_transportation_space_traits!(Road);
crate::impl_abstract_transportation_space_mut_traits!(Road);
crate::impl_has_feature_type!(Road, Road);

impl IterFeatures for Road {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(self.abstract_transportation_space.iter_features())
                .chain(
                    self.sections
                        .iter()
                        .flat_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                )
                .chain(
                    self.intersections
                        .iter()
                        .flat_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                ),
        )
    }
}

impl ForEachFeatureMut for Road {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_transportation_space.for_each_feature_mut(f);
        for prop in &mut self.sections {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.intersections {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for Road {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_transportation_space.compute_envelope()
    }
}

impl ApplyTransform for Road {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_transportation_space.apply_transform(m);

        self.sections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_transform(m));
        self.intersections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_transform(m));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_transportation_space.apply_isometry(isometry);

        self.sections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_isometry(isometry));
        self.intersections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_transportation_space.apply_translation(vector);

        self.sections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_translation(vector));
        self.intersections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_transportation_space.apply_rotation(rotation);

        self.sections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_rotation(rotation));
        self.intersections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_transportation_space.apply_scale(scale);

        self.sections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_scale(scale));
        self.intersections
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_scale(scale));
    }
}
