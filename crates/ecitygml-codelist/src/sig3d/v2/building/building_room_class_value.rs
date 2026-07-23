use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `Room_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/building/2.0/Room_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dBuildingRoomClassValue {
    Habitation,
    Administration,
    BusinessTrade,
    Catering,
    Recreation,
    ChurchInstitution,
    AgricultureForestry,
    SchoolsEducationResearch,
    AccommodationWasteManagement,
    Healthcare,
    Communicating,
    Security,
    Store,
    Industry,
    Traffic,
    Function,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dBuildingRoomClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/Room_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Habitation,
            "1010" => Self::Administration,
            "1020" => Self::BusinessTrade,
            "1030" => Self::Catering,
            "1040" => Self::Recreation,
            "1050" => Self::ChurchInstitution,
            "1060" => Self::AgricultureForestry,
            "1070" => Self::SchoolsEducationResearch,
            "1080" => Self::AccommodationWasteManagement,
            "1090" => Self::Healthcare,
            "1100" => Self::Communicating,
            "1110" => Self::Security,
            "1120" => Self::Store,
            "1130" => Self::Industry,
            "1140" => Self::Traffic,
            "1150" => Self::Function,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Habitation => "1000",
            Self::Administration => "1010",
            Self::BusinessTrade => "1020",
            Self::Catering => "1030",
            Self::Recreation => "1040",
            Self::ChurchInstitution => "1050",
            Self::AgricultureForestry => "1060",
            Self::SchoolsEducationResearch => "1070",
            Self::AccommodationWasteManagement => "1080",
            Self::Healthcare => "1090",
            Self::Communicating => "1100",
            Self::Security => "1110",
            Self::Store => "1120",
            Self::Industry => "1130",
            Self::Traffic => "1140",
            Self::Function => "1150",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::values::BuildingRoomClassValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dBuildingRoomClassValue::Habitation),
            ("1150", Sig3dBuildingRoomClassValue::Function),
        ] {
            assert_eq!(Sig3dBuildingRoomClassValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dBuildingRoomClassValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dBuildingRoomClassValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = BuildingRoomClassValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dBuildingRoomClassValue>(),
            Some(Sig3dBuildingRoomClassValue::Habitation)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = BuildingRoomClassValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dBuildingRoomClassValue>(), None);
    }
}
