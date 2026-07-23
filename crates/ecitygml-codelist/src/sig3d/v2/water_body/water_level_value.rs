use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `WaterSurface_waterLevel.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/waterbody/2.0/WaterSurface_waterLevel.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dWaterLevelValue {
    MslMeanSeaLevel,
    LatLowestAstronomicalTide,
    NationalWaterLevel,
    MeanHighTideRelatedToNationalWaterlevel,
    ExtremeHighTideRelatedToNationalWaterlevel,
    MeanLowTideRelatedToNationalWaterlevel,
    ExtremeLowTideRelatedToNationalWaterlevel,
    MeanWaterLevelWatercourse,
    CriticalHighWaterLevel,
    HundredYearFlood,
    HighestKnownWaterLevel,
    CriticalLowWaterLevel,
    LowestKnownWaterLevel,
    EstablishedLineOfNavigability,
    MinimumLimitOfNavigability,
    MaximumLimitOfNavigability,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dWaterLevelValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/waterbody/2.0/WaterSurface_waterLevel.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::MslMeanSeaLevel,
            "1010" => Self::LatLowestAstronomicalTide,
            "1020" => Self::NationalWaterLevel,
            "1030" => Self::MeanHighTideRelatedToNationalWaterlevel,
            "1040" => Self::ExtremeHighTideRelatedToNationalWaterlevel,
            "1050" => Self::MeanLowTideRelatedToNationalWaterlevel,
            "1060" => Self::ExtremeLowTideRelatedToNationalWaterlevel,
            "1070" => Self::MeanWaterLevelWatercourse,
            "1080" => Self::CriticalHighWaterLevel,
            "1090" => Self::HundredYearFlood,
            "1100" => Self::HighestKnownWaterLevel,
            "1110" => Self::CriticalLowWaterLevel,
            "1120" => Self::LowestKnownWaterLevel,
            "1130" => Self::EstablishedLineOfNavigability,
            "1140" => Self::MinimumLimitOfNavigability,
            "1150" => Self::MaximumLimitOfNavigability,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::MslMeanSeaLevel => "1000",
            Self::LatLowestAstronomicalTide => "1010",
            Self::NationalWaterLevel => "1020",
            Self::MeanHighTideRelatedToNationalWaterlevel => "1030",
            Self::ExtremeHighTideRelatedToNationalWaterlevel => "1040",
            Self::MeanLowTideRelatedToNationalWaterlevel => "1050",
            Self::ExtremeLowTideRelatedToNationalWaterlevel => "1060",
            Self::MeanWaterLevelWatercourse => "1070",
            Self::CriticalHighWaterLevel => "1080",
            Self::HundredYearFlood => "1090",
            Self::HighestKnownWaterLevel => "1100",
            Self::CriticalLowWaterLevel => "1110",
            Self::LowestKnownWaterLevel => "1120",
            Self::EstablishedLineOfNavigability => "1130",
            Self::MinimumLimitOfNavigability => "1140",
            Self::MaximumLimitOfNavigability => "1150",
            Self::Unknown => "9999",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::water_body::values::WaterLevelValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dWaterLevelValue::MslMeanSeaLevel),
            ("9999", Sig3dWaterLevelValue::Unknown),
        ] {
            assert_eq!(Sig3dWaterLevelValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dWaterLevelValue::from_code_value("999999");
        assert_eq!(value, Sig3dWaterLevelValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = WaterLevelValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dWaterLevelValue>(),
            Some(Sig3dWaterLevelValue::MslMeanSeaLevel)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = WaterLevelValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dWaterLevelValue>(), None);
    }
}
