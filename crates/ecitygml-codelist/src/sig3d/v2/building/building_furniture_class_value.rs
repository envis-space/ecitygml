use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `BuildingFurniture_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/building/2.0/BuildingFurniture_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dBuildingFurnitureClassValue {
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
    MaintenanceWasteManagement,
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

impl CodeListValue for Sig3dBuildingFurnitureClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/BuildingFurniture_class.xml";

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
            "1110" => Self::MaintenanceWasteManagement,
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
            Self::MaintenanceWasteManagement => "1110",
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

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dBuildingFurnitureClassValue::Habitation),
            ("1180", Sig3dBuildingFurnitureClassValue::Function),
        ] {
            assert_eq!(
                Sig3dBuildingFurnitureClassValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dBuildingFurnitureClassValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dBuildingFurnitureClassValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }
}
