use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `_AbstractBuilding_class.xml` (version 2.0),
/// used to classify `Building`/`BuildingPart` features.
///
/// <https://www.sig3d.de/codelists/standard/building/2.0/_AbstractBuilding_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dBuildingClassValue {
    Habitation,
    Sanitation,
    Administration,
    BusinessTrade,
    Catering,
    Recreation,
    Sport,
    Culture,
    ChurchInstitution,
    AgricultureForestry,
    SchoolsEducationResearch,
    MaintenanceAndWasteManagement,
    Healthcare,
    Communicating,
    Security,
    Storage,
    Industry,
    Traffic,
    Function,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dBuildingClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Habitation,
            "1010" => Self::Sanitation,
            "1020" => Self::Administration,
            "1030" => Self::BusinessTrade,
            "1040" => Self::Catering,
            "1050" => Self::Recreation,
            "1060" => Self::Sport,
            "1070" => Self::Culture,
            "1080" => Self::ChurchInstitution,
            "1090" => Self::AgricultureForestry,
            "1100" => Self::SchoolsEducationResearch,
            "1110" => Self::MaintenanceAndWasteManagement,
            "1120" => Self::Healthcare,
            "1130" => Self::Communicating,
            "1140" => Self::Security,
            "1150" => Self::Storage,
            "1160" => Self::Industry,
            "1170" => Self::Traffic,
            "1180" => Self::Function,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Habitation => "1000",
            Self::Sanitation => "1010",
            Self::Administration => "1020",
            Self::BusinessTrade => "1030",
            Self::Catering => "1040",
            Self::Recreation => "1050",
            Self::Sport => "1060",
            Self::Culture => "1070",
            Self::ChurchInstitution => "1080",
            Self::AgricultureForestry => "1090",
            Self::SchoolsEducationResearch => "1100",
            Self::MaintenanceAndWasteManagement => "1110",
            Self::Healthcare => "1120",
            Self::Communicating => "1130",
            Self::Security => "1140",
            Self::Storage => "1150",
            Self::Industry => "1160",
            Self::Traffic => "1170",
            Self::Function => "1180",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::values::BuildingClassValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dBuildingClassValue::Habitation),
            ("1090", Sig3dBuildingClassValue::AgricultureForestry),
            ("1180", Sig3dBuildingClassValue::Function),
        ] {
            assert_eq!(Sig3dBuildingClassValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dBuildingClassValue::from_code_value("9999");
        assert_eq!(value, Sig3dBuildingClassValue::Other("9999".to_string()));
        assert_eq!(value.to_code_value(), "9999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = BuildingClassValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dBuildingClassValue>(),
            Some(Sig3dBuildingClassValue::Habitation)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = BuildingClassValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<Sig3dBuildingClassValue>(), None);
    }
}
