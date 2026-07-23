use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `TrafficArea_usage.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/transportation/2.0/TrafficArea_usage.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dTrafficAreaUsageValue {
    Pedestrian,
    Car,
    Truck,
    BusTaxi,
    Train,
    Bicycle,
    Motorcycle,
    TramStreetcar,
    BoatFerryShip,
    Teleferic,
    Aeroplane,
    Helicopter,
    Taxi,
    Horse,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dTrafficAreaUsageValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/transportation/2.0/TrafficArea_usage.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1" => Self::Pedestrian,
            "2" => Self::Car,
            "3" => Self::Truck,
            "4" => Self::BusTaxi,
            "5" => Self::Train,
            "6" => Self::Bicycle,
            "7" => Self::Motorcycle,
            "8" => Self::TramStreetcar,
            "9" => Self::BoatFerryShip,
            "10" => Self::Teleferic,
            "11" => Self::Aeroplane,
            "12" => Self::Helicopter,
            "13" => Self::Taxi,
            "14" => Self::Horse,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Pedestrian => "1",
            Self::Car => "2",
            Self::Truck => "3",
            Self::BusTaxi => "4",
            Self::Train => "5",
            Self::Bicycle => "6",
            Self::Motorcycle => "7",
            Self::TramStreetcar => "8",
            Self::BoatFerryShip => "9",
            Self::Teleferic => "10",
            Self::Aeroplane => "11",
            Self::Helicopter => "12",
            Self::Taxi => "13",
            Self::Horse => "14",
            Self::Unknown => "9999",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::transportation::values::TrafficAreaUsageValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1", Sig3dTrafficAreaUsageValue::Pedestrian),
            ("9999", Sig3dTrafficAreaUsageValue::Unknown),
        ] {
            assert_eq!(Sig3dTrafficAreaUsageValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dTrafficAreaUsageValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dTrafficAreaUsageValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = TrafficAreaUsageValue::from(Code::new("1"));
        assert_eq!(
            wrapped.interpret::<Sig3dTrafficAreaUsageValue>(),
            Some(Sig3dTrafficAreaUsageValue::Pedestrian)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = TrafficAreaUsageValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1",
        ));
        assert_eq!(wrapped.interpret::<Sig3dTrafficAreaUsageValue>(), None);
    }
}
