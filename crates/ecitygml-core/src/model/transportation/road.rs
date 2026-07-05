use crate::model::core::AsAbstractFeatureMut;
use crate::model::core::refs::FeatureKindRef;
use crate::model::core::refs::FeatureKindRefMut;
use crate::model::transportation::{
    AbstractTransportationSpace, AsAbstractTransportationSpace, AsAbstractTransportationSpaceMut,
    IntersectionProperty, SectionProperty,
};
use egml::model::base::Id;
use egml::model::basic::Code;
use egml::model::geometry::Envelope;
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq)]
pub struct Road {
    pub(crate) abstract_transportation_space: AbstractTransportationSpace,
    class: Option<Code>,
    functions: Vec<Code>,
    usages: Vec<Code>,
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

    pub fn class(&self) -> Option<&Code> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: Option<Code>) {
        self.class = class;
    }

    pub fn functions(&self) -> &[Code] {
        &self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<Code>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: Code) {
        self.functions.push(function);
    }

    pub fn extend_functions(&mut self, functions: impl IntoIterator<Item = Code>) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[Code] {
        &self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<Code>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: Code) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = Code>) {
        self.usages.extend(usages);
    }

    pub fn sections(&self) -> &[SectionProperty] {
        &self.sections
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

impl Road {
    pub fn iter_features<'a>(&'a self) -> impl Iterator<Item = FeatureKindRef<'a>> + 'a {
        std::iter::once(self.into())
            .chain(self.abstract_transportation_space.iter_features())
            .chain(
                self.sections
                    .iter()
                    .flat_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
            .chain(
                self.intersections
                    .iter()
                    .flat_map(|x| x.object.as_ref())
                    .flat_map(|x| x.iter_features()),
            )
    }

    pub fn for_each_feature_mut<F: FnMut(FeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_transportation_space.for_each_feature_mut(f);
        for prop in &mut self.sections {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
        for prop in &mut self.intersections {
            if let Some(x) = prop.object.as_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_transportation_space.compute_envelope()
    }

    pub fn recompute_bounding_shape(&mut self) {
        self.set_bounding_shape_from_envelope(self.compute_envelope());
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.abstract_transportation_space.apply_transform(m);

        self.sections
            .iter_mut()
            .flat_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
        self.intersections
            .iter_mut()
            .flat_map(|x| x.object.as_mut())
            .for_each(|x| x.apply_transform(m));
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
