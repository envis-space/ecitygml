use crate::model::common::{ForEachFeatureMut, IterFeatures};
use crate::model::core::refs::{AbstractFeatureKindRef, AbstractFeatureKindRefMut};
use crate::model::core::{
    AbstractUnoccupiedSpace, AsAbstractUnoccupiedSpace, AsAbstractUnoccupiedSpaceMut, Occupancy,
};
use crate::model::transportation::TrafficSpaceReference;
use crate::model::transportation::granularity_value::GranularityValue;
use crate::model::transportation::values::{
    TrafficSpaceClassValue, TrafficSpaceFunctionValue, TrafficSpaceUsageValue,
};
use crate::model::transportation::{ClearanceSpaceProperty, TrafficDirectionValue};
use egml::model::base::Id;
use egml::model::common::{ApplyTransform, ComputeEnvelope};
use egml::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub struct TrafficSpace {
    pub(crate) abstract_unoccupied_space: AbstractUnoccupiedSpace,
    class: Option<TrafficSpaceClassValue>,
    functions: Vec<TrafficSpaceFunctionValue>,
    usages: Vec<TrafficSpaceUsageValue>,
    granularity: GranularityValue,
    traffic_direction: Option<TrafficDirectionValue>,
    occupancies: Vec<Occupancy>,
    predecessors: Vec<TrafficSpaceReference>,
    successors: Vec<TrafficSpaceReference>,
    clearance_spaces: Vec<ClearanceSpaceProperty>,
}

impl TrafficSpace {
    pub fn new(id: Id, granularity: GranularityValue) -> Self {
        Self::from_abstract_unoccupied_space(AbstractUnoccupiedSpace::new(id), granularity)
    }

    pub fn from_abstract_unoccupied_space(
        abstract_unoccupied_space: AbstractUnoccupiedSpace,
        granularity: GranularityValue,
    ) -> Self {
        Self {
            abstract_unoccupied_space,
            class: None,
            functions: Vec::new(),
            usages: Vec::new(),
            granularity,
            traffic_direction: None,
            occupancies: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
            clearance_spaces: Vec::new(),
        }
    }

    pub fn class(&self) -> Option<&TrafficSpaceClassValue> {
        self.class.as_ref()
    }

    pub fn set_class(&mut self, class: TrafficSpaceClassValue) {
        self.class = Some(class);
    }

    pub fn set_class_opt(&mut self, class: Option<TrafficSpaceClassValue>) {
        self.class = class;
    }

    pub fn clear_class(&mut self) {
        self.class = None;
    }

    pub fn functions(&self) -> &[TrafficSpaceFunctionValue] {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [TrafficSpaceFunctionValue] {
        &mut self.functions
    }

    pub fn set_functions(&mut self, functions: Vec<TrafficSpaceFunctionValue>) {
        self.functions = functions;
    }

    pub fn push_function(&mut self, function: TrafficSpaceFunctionValue) {
        self.functions.push(function);
    }

    pub fn extend_functions(
        &mut self,
        functions: impl IntoIterator<Item = TrafficSpaceFunctionValue>,
    ) {
        self.functions.extend(functions);
    }

    pub fn usages(&self) -> &[TrafficSpaceUsageValue] {
        &self.usages
    }

    pub fn usages_mut(&mut self) -> &mut [TrafficSpaceUsageValue] {
        &mut self.usages
    }

    pub fn set_usages(&mut self, usages: Vec<TrafficSpaceUsageValue>) {
        self.usages = usages;
    }

    pub fn push_usage(&mut self, usage: TrafficSpaceUsageValue) {
        self.usages.push(usage);
    }

    pub fn extend_usages(&mut self, usages: impl IntoIterator<Item = TrafficSpaceUsageValue>) {
        self.usages.extend(usages);
    }

    pub fn granularity(&self) -> &GranularityValue {
        &self.granularity
    }

    pub fn set_granularity(&mut self, granularity: GranularityValue) {
        self.granularity = granularity;
    }

    pub fn traffic_direction(&self) -> Option<TrafficDirectionValue> {
        self.traffic_direction
    }

    pub fn set_traffic_direction(&mut self, traffic_direction: TrafficDirectionValue) {
        self.traffic_direction = Some(traffic_direction);
    }

    pub fn set_traffic_direction_opt(&mut self, traffic_direction: Option<TrafficDirectionValue>) {
        self.traffic_direction = traffic_direction;
    }

    pub fn clear_traffic_direction(&mut self) {
        self.traffic_direction = None;
    }

    pub fn occupancies(&self) -> &[Occupancy] {
        &self.occupancies
    }

    pub fn occupancies_mut(&mut self) -> &mut [Occupancy] {
        &mut self.occupancies
    }

    pub fn set_occupancies(&mut self, occupancies: Vec<Occupancy>) {
        self.occupancies = occupancies;
    }

    pub fn push_occupancy(&mut self, occupancy: Occupancy) {
        self.occupancies.push(occupancy);
    }

    pub fn extend_occupancies(&mut self, occupancies: impl IntoIterator<Item = Occupancy>) {
        self.occupancies.extend(occupancies);
    }

    pub fn predecessors(&self) -> &[TrafficSpaceReference] {
        &self.predecessors
    }

    pub fn predecessors_mut(&mut self) -> &mut [TrafficSpaceReference] {
        &mut self.predecessors
    }

    pub fn set_predecessors(&mut self, predecessors: Vec<TrafficSpaceReference>) {
        self.predecessors = predecessors;
    }

    pub fn push_predecessor(&mut self, predecessor: TrafficSpaceReference) {
        self.predecessors.push(predecessor);
    }

    pub fn extend_predecessors(
        &mut self,
        predecessors: impl IntoIterator<Item = TrafficSpaceReference>,
    ) {
        self.predecessors.extend(predecessors);
    }

    pub fn successors(&self) -> &[TrafficSpaceReference] {
        &self.successors
    }

    pub fn successors_mut(&mut self) -> &mut [TrafficSpaceReference] {
        &mut self.successors
    }

    pub fn set_successors(&mut self, successors: Vec<TrafficSpaceReference>) {
        self.successors = successors;
    }

    pub fn push_successor(&mut self, successor: TrafficSpaceReference) {
        self.successors.push(successor);
    }

    pub fn extend_successors(
        &mut self,
        successors: impl IntoIterator<Item = TrafficSpaceReference>,
    ) {
        self.successors.extend(successors);
    }

    pub fn clearance_spaces(&self) -> &[ClearanceSpaceProperty] {
        &self.clearance_spaces
    }

    pub fn clearance_spaces_mut(&mut self) -> &mut [ClearanceSpaceProperty] {
        &mut self.clearance_spaces
    }

    pub fn set_clearance_spaces(&mut self, clearance_spaces: Vec<ClearanceSpaceProperty>) {
        self.clearance_spaces = clearance_spaces;
    }

    pub fn push_clearance_space(&mut self, clearance_space: ClearanceSpaceProperty) {
        self.clearance_spaces.push(clearance_space);
    }

    pub fn extend_clearance_spaces(
        &mut self,
        clearance_spaces: impl IntoIterator<Item = ClearanceSpaceProperty>,
    ) {
        self.clearance_spaces.extend(clearance_spaces);
    }
}

impl AsAbstractUnoccupiedSpace for TrafficSpace {
    fn abstract_unoccupied_space(&self) -> &AbstractUnoccupiedSpace {
        &self.abstract_unoccupied_space
    }
}

impl AsAbstractUnoccupiedSpaceMut for TrafficSpace {
    fn abstract_unoccupied_space_mut(&mut self) -> &mut AbstractUnoccupiedSpace {
        &mut self.abstract_unoccupied_space
    }
}

crate::impl_abstract_unoccupied_space_traits!(TrafficSpace);
crate::impl_abstract_unoccupied_space_mut_traits!(TrafficSpace);
crate::impl_has_feature_type!(TrafficSpace, TrafficSpace);

impl IterFeatures for TrafficSpace {
    fn iter_features(&self) -> Box<dyn Iterator<Item = AbstractFeatureKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into())
                .chain(self.abstract_unoccupied_space.iter_features())
                .chain(
                    self.clearance_spaces
                        .iter()
                        .flat_map(|x| x.object())
                        .flat_map(|x| x.iter_features()),
                ),
        )
    }
}

impl ForEachFeatureMut for TrafficSpace {
    fn for_each_feature_mut<F: FnMut(AbstractFeatureKindRefMut<'_>)>(&mut self, f: &mut F) {
        f((&mut *self).into());
        self.abstract_unoccupied_space.for_each_feature_mut(f);
        for prop in &mut self.clearance_spaces {
            if let Some(x) = prop.object_mut() {
                x.for_each_feature_mut(f);
            }
        }
    }
}

impl ComputeEnvelope for TrafficSpace {
    fn compute_envelope(&self) -> Option<Envelope> {
        self.abstract_unoccupied_space.compute_envelope()
    }
}

impl ApplyTransform for TrafficSpace {
    fn apply_transform(&mut self, m: Transform3<f64>) {
        self.abstract_unoccupied_space.apply_transform(m);

        self.clearance_spaces
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_transform(m));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.abstract_unoccupied_space.apply_isometry(isometry);

        self.clearance_spaces
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.abstract_unoccupied_space.apply_translation(vector);

        self.clearance_spaces
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.abstract_unoccupied_space.apply_rotation(rotation);

        self.clearance_spaces
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.abstract_unoccupied_space.apply_scale(scale);

        self.clearance_spaces
            .iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_scale(scale));
    }
}
