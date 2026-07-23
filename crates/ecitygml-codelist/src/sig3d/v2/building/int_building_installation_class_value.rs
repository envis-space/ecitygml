use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `IntBuildingInstallation_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/building/2.0/IntBuildingInstallation_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dIntBuildingInstallationClassValue {
    HeatingVentilationClimate,
    Safety,
    Illumination,
    Communication,
    SupplyAndDisposal,
    Statics,
    Entertainmant,
    Miscellaneous,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dIntBuildingInstallationClassValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/IntBuildingInstallation_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::HeatingVentilationClimate,
            "2000" => Self::Safety,
            "3000" => Self::Illumination,
            "4000" => Self::Communication,
            "5000" => Self::SupplyAndDisposal,
            "6000" => Self::Statics,
            "7000" => Self::Entertainmant,
            "8000" => Self::Miscellaneous,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::HeatingVentilationClimate => "1000",
            Self::Safety => "2000",
            Self::Illumination => "3000",
            Self::Communication => "4000",
            Self::SupplyAndDisposal => "5000",
            Self::Statics => "6000",
            Self::Entertainmant => "7000",
            Self::Miscellaneous => "8000",
            Self::Unknown => "9999",
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
            (
                "1000",
                Sig3dIntBuildingInstallationClassValue::HeatingVentilationClimate,
            ),
            ("9999", Sig3dIntBuildingInstallationClassValue::Unknown),
        ] {
            assert_eq!(
                Sig3dIntBuildingInstallationClassValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dIntBuildingInstallationClassValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dIntBuildingInstallationClassValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }
}
