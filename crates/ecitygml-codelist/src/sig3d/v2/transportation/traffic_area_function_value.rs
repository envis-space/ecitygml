use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `TrafficArea_function.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/transportation/2.0/TrafficArea_function.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dTrafficAreaFunctionValue {
    DrivingLane,
    Footpath,
    Cyclepath,
    CombinedFootCyclepath,
    Square,
    CarPark,
    ParkingLayBy,
    Rail,
    RailRoadCombined,
    Drainage,
    RoadMarking,
    RoadMarkingDirection,
    RoadMarkingLane,
    RoadMarkingRestricted,
    RoadMarkingCrosswalk,
    RoadMarkingStop,
    RoadMarkingOther,
    OverheadWireTrolley,
    TrainPlatform,
    Crosswalk,
    Barrier,
    Stairs,
    Escalator,
    FilteringLane,
    AirportRunway,
    AirportTaxiway,
    AirportApron,
    AirportHeliport,
    AirportRunwayMarking,
    GreenSpaces,
    Recreation,
    BusLayBy,
    Motorway,
    MotorwayEntry,
    MotorwayExit,
    MotorwayEmergencyLane,
    PrivateArea,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dTrafficAreaFunctionValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/transportation/2.0/TrafficArea_function.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1" => Self::DrivingLane,
            "2" => Self::Footpath,
            "3" => Self::Cyclepath,
            "4" => Self::CombinedFootCyclepath,
            "5" => Self::Square,
            "6" => Self::CarPark,
            "7" => Self::ParkingLayBy,
            "8" => Self::Rail,
            "9" => Self::RailRoadCombined,
            "10" => Self::Drainage,
            "11" => Self::RoadMarking,
            "12" => Self::RoadMarkingDirection,
            "13" => Self::RoadMarkingLane,
            "14" => Self::RoadMarkingRestricted,
            "15" => Self::RoadMarkingCrosswalk,
            "16" => Self::RoadMarkingStop,
            "17" => Self::RoadMarkingOther,
            "18" => Self::OverheadWireTrolley,
            "19" => Self::TrainPlatform,
            "20" => Self::Crosswalk,
            "21" => Self::Barrier,
            "22" => Self::Stairs,
            "23" => Self::Escalator,
            "24" => Self::FilteringLane,
            "25" => Self::AirportRunway,
            "26" => Self::AirportTaxiway,
            "27" => Self::AirportApron,
            "28" => Self::AirportHeliport,
            "29" => Self::AirportRunwayMarking,
            "30" => Self::GreenSpaces,
            "31" => Self::Recreation,
            "32" => Self::BusLayBy,
            "33" => Self::Motorway,
            "34" => Self::MotorwayEntry,
            "35" => Self::MotorwayExit,
            "36" => Self::MotorwayEmergencyLane,
            "37" => Self::PrivateArea,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::DrivingLane => "1",
            Self::Footpath => "2",
            Self::Cyclepath => "3",
            Self::CombinedFootCyclepath => "4",
            Self::Square => "5",
            Self::CarPark => "6",
            Self::ParkingLayBy => "7",
            Self::Rail => "8",
            Self::RailRoadCombined => "9",
            Self::Drainage => "10",
            Self::RoadMarking => "11",
            Self::RoadMarkingDirection => "12",
            Self::RoadMarkingLane => "13",
            Self::RoadMarkingRestricted => "14",
            Self::RoadMarkingCrosswalk => "15",
            Self::RoadMarkingStop => "16",
            Self::RoadMarkingOther => "17",
            Self::OverheadWireTrolley => "18",
            Self::TrainPlatform => "19",
            Self::Crosswalk => "20",
            Self::Barrier => "21",
            Self::Stairs => "22",
            Self::Escalator => "23",
            Self::FilteringLane => "24",
            Self::AirportRunway => "25",
            Self::AirportTaxiway => "26",
            Self::AirportApron => "27",
            Self::AirportHeliport => "28",
            Self::AirportRunwayMarking => "29",
            Self::GreenSpaces => "30",
            Self::Recreation => "31",
            Self::BusLayBy => "32",
            Self::Motorway => "33",
            Self::MotorwayEntry => "34",
            Self::MotorwayExit => "35",
            Self::MotorwayEmergencyLane => "36",
            Self::PrivateArea => "37",
            Self::Unknown => "9999",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::transportation::values::TrafficAreaFunctionValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1", Sig3dTrafficAreaFunctionValue::DrivingLane),
            ("9999", Sig3dTrafficAreaFunctionValue::Unknown),
        ] {
            assert_eq!(Sig3dTrafficAreaFunctionValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dTrafficAreaFunctionValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dTrafficAreaFunctionValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = TrafficAreaFunctionValue::from(Code::new("1"));
        assert_eq!(
            wrapped.interpret::<Sig3dTrafficAreaFunctionValue>(),
            Some(Sig3dTrafficAreaFunctionValue::DrivingLane)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = TrafficAreaFunctionValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1",
        ));
        assert_eq!(wrapped.interpret::<Sig3dTrafficAreaFunctionValue>(), None);
    }
}
