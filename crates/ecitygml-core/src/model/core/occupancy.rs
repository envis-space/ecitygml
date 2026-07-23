use crate::model::core::values::IntervalValue;
use crate::model::core::values::OccupantTypeValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Occupancy {
    number_of_occupants: i64,
    interval: Option<IntervalValue>,
    occupant_type: Option<OccupantTypeValue>,
}

impl Occupancy {
    pub fn new(number_of_occupants: i64) -> Self {
        Self {
            number_of_occupants,
            interval: None,
            occupant_type: None,
        }
    }

    pub fn number_of_occupants(&self) -> i64 {
        self.number_of_occupants
    }

    pub fn set_number_of_occupants(&mut self, value: i64) {
        self.number_of_occupants = value;
    }

    pub fn interval(&self) -> Option<&IntervalValue> {
        self.interval.as_ref()
    }

    pub fn set_interval(&mut self, value: IntervalValue) {
        self.interval = Some(value);
    }

    pub fn set_interval_opt(&mut self, value: Option<IntervalValue>) {
        self.interval = value;
    }

    pub fn occupant_type(&self) -> Option<&OccupantTypeValue> {
        self.occupant_type.as_ref()
    }

    pub fn set_occupant_type(&mut self, value: OccupantTypeValue) {
        self.occupant_type = Some(value);
    }

    pub fn set_occupant_type_opt(&mut self, value: Option<OccupantTypeValue>) {
        self.occupant_type = value;
    }
}
