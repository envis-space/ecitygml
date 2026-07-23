use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `BuildingInstallation_usage.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/building/2.0/BuildingInstallation_usage.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dBuildingInstallationUsageValue {
    Balcony,
    WinterGarden,
    Arcade,
    ChimneyPartOfABuilding,
    TowerPartOfABuilding,
    Column,
    Stairs,
    Others,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dBuildingInstallationUsageValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/BuildingInstallation_usage.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Balcony,
            "1010" => Self::WinterGarden,
            "1020" => Self::Arcade,
            "1030" => Self::ChimneyPartOfABuilding,
            "1040" => Self::TowerPartOfABuilding,
            "1050" => Self::Column,
            "1060" => Self::Stairs,
            "1070" => Self::Others,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Balcony => "1000",
            Self::WinterGarden => "1010",
            Self::Arcade => "1020",
            Self::ChimneyPartOfABuilding => "1030",
            Self::TowerPartOfABuilding => "1040",
            Self::Column => "1050",
            Self::Stairs => "1060",
            Self::Others => "1070",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::values::BuildingInstallationUsageValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1000", Sig3dBuildingInstallationUsageValue::Balcony),
            ("1070", Sig3dBuildingInstallationUsageValue::Others),
        ] {
            assert_eq!(
                Sig3dBuildingInstallationUsageValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dBuildingInstallationUsageValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dBuildingInstallationUsageValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = BuildingInstallationUsageValue::from(Code::new("1000"));
        assert_eq!(
            wrapped.interpret::<Sig3dBuildingInstallationUsageValue>(),
            Some(Sig3dBuildingInstallationUsageValue::Balcony)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = BuildingInstallationUsageValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(
            wrapped.interpret::<Sig3dBuildingInstallationUsageValue>(),
            None
        );
    }
}
