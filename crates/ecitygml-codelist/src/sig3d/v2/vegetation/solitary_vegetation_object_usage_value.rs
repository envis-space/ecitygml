use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `SolitaryVegetationObject_usage.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/vegetation/2.0/SolitaryVegetationObject_usage.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dSolitaryVegetationObjectUsageValue {
    Shrub,
    LowPlants,
    MediumHighPlants,
    HighPlants,
    Grasses,
    Ferns,
    ConiferousTree,
    DecidousTree,
    Bushes,
    AquaticPlants,
    Climber,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dSolitaryVegetationObjectUsageValue {
    const CODE_SPACE: &'static str = "http://www.sig3d.org/codelists/citygml/2.0/vegetation/2.0/SolitaryVegetationObject_usage.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Shrub,
            "1010" => Self::LowPlants,
            "1020" => Self::MediumHighPlants,
            "1030" => Self::HighPlants,
            "1040" => Self::Grasses,
            "1050" => Self::Ferns,
            "1060" => Self::ConiferousTree,
            "1070" => Self::DecidousTree,
            "1080" => Self::Bushes,
            "1090" => Self::AquaticPlants,
            "1100" => Self::Climber,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Shrub => "1000",
            Self::LowPlants => "1010",
            Self::MediumHighPlants => "1020",
            Self::HighPlants => "1030",
            Self::Grasses => "1040",
            Self::Ferns => "1050",
            Self::ConiferousTree => "1060",
            Self::DecidousTree => "1070",
            Self::Bushes => "1080",
            Self::AquaticPlants => "1090",
            Self::Climber => "1100",
            Self::Unknown => "9999",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::vegetation::values::SolitaryVegetationObjectUsageValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dSolitaryVegetationObjectUsageValue::Shrub),
            ("9999", Sig3dSolitaryVegetationObjectUsageValue::Unknown),
        ] {
            assert_eq!(
                Sig3dSolitaryVegetationObjectUsageValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dSolitaryVegetationObjectUsageValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dSolitaryVegetationObjectUsageValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = SolitaryVegetationObjectUsageValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dSolitaryVegetationObjectUsageValue>(),
            Some(Sig3dSolitaryVegetationObjectUsageValue::Shrub)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = SolitaryVegetationObjectUsageValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(
            wrapped.interpret::<Sig3dSolitaryVegetationObjectUsageValue>(),
            None
        );
    }
}
