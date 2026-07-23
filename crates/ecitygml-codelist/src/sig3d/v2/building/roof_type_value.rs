use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `_AbstractBuilding_roofType.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/building/2.0/_AbstractBuilding_roofType.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dRoofTypeValue {
    FlatRoof,
    MonopitchRoof,
    DualPentRoof,
    GabledRoof,
    HippedRoof,
    HalfHippedRoof,
    MansardRoof,
    PavilionRoof,
    ConeRoof,
    CopulaRoof,
    SawtoothRoof,
    ArchRoof,
    PyramidalBroachRoof,
    CombinationOfRoofForms,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dRoofTypeValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_roofType.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::FlatRoof,
            "1010" => Self::MonopitchRoof,
            "1020" => Self::DualPentRoof,
            "1030" => Self::GabledRoof,
            "1040" => Self::HippedRoof,
            "1050" => Self::HalfHippedRoof,
            "1060" => Self::MansardRoof,
            "1070" => Self::PavilionRoof,
            "1080" => Self::ConeRoof,
            "1090" => Self::CopulaRoof,
            "1100" => Self::SawtoothRoof,
            "1110" => Self::ArchRoof,
            "1120" => Self::PyramidalBroachRoof,
            "1130" => Self::CombinationOfRoofForms,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::FlatRoof => "1000",
            Self::MonopitchRoof => "1010",
            Self::DualPentRoof => "1020",
            Self::GabledRoof => "1030",
            Self::HippedRoof => "1040",
            Self::HalfHippedRoof => "1050",
            Self::MansardRoof => "1060",
            Self::PavilionRoof => "1070",
            Self::ConeRoof => "1080",
            Self::CopulaRoof => "1090",
            Self::SawtoothRoof => "1100",
            Self::ArchRoof => "1110",
            Self::PyramidalBroachRoof => "1120",
            Self::CombinationOfRoofForms => "1130",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::values::RoofTypeValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dRoofTypeValue::FlatRoof),
            ("1130", Sig3dRoofTypeValue::CombinationOfRoofForms),
        ] {
            assert_eq!(Sig3dRoofTypeValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dRoofTypeValue::from_code_value("999999");
        assert_eq!(value, Sig3dRoofTypeValue::Other("999999".to_string()));
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = RoofTypeValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dRoofTypeValue>(),
            Some(Sig3dRoofTypeValue::FlatRoof)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = RoofTypeValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dRoofTypeValue>(), None);
    }
}
