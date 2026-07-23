use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `WaterBody_function.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/waterbody/2.0/WaterBody_function.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dWaterBodyFunctionValue {
    NatureSanctuary,
    ProtectedWaterbody,
    Reservoir,
    RetentionWaterbody,
    FloodPlainWaterbody,
    Waterway,
    HaborWaterbody,
    SluiceWaterbody,
    SewageSystem,
    PublicSwimming,
    PublicFountain,
    PrivateWaterbody,
    IrrigationWaterbody,
    WateringPlace,
    IndustrialWaterbody,
    WaterbodyForFireFighting,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dWaterBodyFunctionValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/waterbody/2.0/WaterBody_function.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::NatureSanctuary,
            "1010" => Self::ProtectedWaterbody,
            "1020" => Self::Reservoir,
            "1030" => Self::RetentionWaterbody,
            "1040" => Self::FloodPlainWaterbody,
            "1050" => Self::Waterway,
            "1060" => Self::HaborWaterbody,
            "1070" => Self::SluiceWaterbody,
            "1080" => Self::SewageSystem,
            "1090" => Self::PublicSwimming,
            "1100" => Self::PublicFountain,
            "1110" => Self::PrivateWaterbody,
            "1120" => Self::IrrigationWaterbody,
            "1130" => Self::WateringPlace,
            "1140" => Self::IndustrialWaterbody,
            "1150" => Self::WaterbodyForFireFighting,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::NatureSanctuary => "1000",
            Self::ProtectedWaterbody => "1010",
            Self::Reservoir => "1020",
            Self::RetentionWaterbody => "1030",
            Self::FloodPlainWaterbody => "1040",
            Self::Waterway => "1050",
            Self::HaborWaterbody => "1060",
            Self::SluiceWaterbody => "1070",
            Self::SewageSystem => "1080",
            Self::PublicSwimming => "1090",
            Self::PublicFountain => "1100",
            Self::PrivateWaterbody => "1110",
            Self::IrrigationWaterbody => "1120",
            Self::WateringPlace => "1130",
            Self::IndustrialWaterbody => "1140",
            Self::WaterbodyForFireFighting => "1150",
            Self::Unknown => "9999",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::water_body::values::WaterBodyFunctionValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dWaterBodyFunctionValue::NatureSanctuary),
            ("9999", Sig3dWaterBodyFunctionValue::Unknown),
        ] {
            assert_eq!(Sig3dWaterBodyFunctionValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dWaterBodyFunctionValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dWaterBodyFunctionValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = WaterBodyFunctionValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dWaterBodyFunctionValue>(),
            Some(Sig3dWaterBodyFunctionValue::NatureSanctuary)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = WaterBodyFunctionValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dWaterBodyFunctionValue>(), None);
    }
}
