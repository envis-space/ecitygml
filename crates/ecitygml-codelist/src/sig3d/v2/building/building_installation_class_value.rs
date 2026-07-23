use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `BuildingInstallation_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/building/2.0/BuildingInstallation_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dBuildingInstallationClassValue {
    OuterCharacteristics,
    InnerCharacteristics,
    WasteManagement,
    Maintenance,
    Communicating,
    Security,
    Others,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dBuildingInstallationClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/BuildingInstallation_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::OuterCharacteristics,
            "1010" => Self::InnerCharacteristics,
            "1020" => Self::WasteManagement,
            "1030" => Self::Maintenance,
            "1040" => Self::Communicating,
            "1050" => Self::Security,
            "1060" => Self::Others,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::OuterCharacteristics => "1000",
            Self::InnerCharacteristics => "1010",
            Self::WasteManagement => "1020",
            Self::Maintenance => "1030",
            Self::Communicating => "1040",
            Self::Security => "1050",
            Self::Others => "1060",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::values::BuildingInstallationClassValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            (
                "1000",
                Sig3dBuildingInstallationClassValue::OuterCharacteristics,
            ),
            ("1060", Sig3dBuildingInstallationClassValue::Others),
        ] {
            assert_eq!(
                Sig3dBuildingInstallationClassValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dBuildingInstallationClassValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dBuildingInstallationClassValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = BuildingInstallationClassValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dBuildingInstallationClassValue>(),
            Some(Sig3dBuildingInstallationClassValue::OuterCharacteristics)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = BuildingInstallationClassValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(
            wrapped.interpret::<Sig3dBuildingInstallationClassValue>(),
            None
        );
    }
}
