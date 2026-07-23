use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `IntBuildingInstallation_function.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/building/2.0/IntBuildingInstallation_function.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dIntBuildingInstallationFunctionValue {
    Radiator,
    Oven,
    Fireside,
    Ventilator,
    AirConditioning,
    Pipe,
    Lamp,
    LightSwitch,
    PowerPoint,
    Cable,
    Rafter,
    Column,
    Railing,
    Stair,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dIntBuildingInstallationFunctionValue {
    const CODE_SPACE: &'static str = "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/IntBuildingInstallation_function.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1010" => Self::Radiator,
            "1020" => Self::Oven,
            "1030" => Self::Fireside,
            "1040" => Self::Ventilator,
            "1050" => Self::AirConditioning,
            "5010" => Self::Pipe,
            "3010" => Self::Lamp,
            "3020" => Self::LightSwitch,
            "5030" => Self::PowerPoint,
            "5020" => Self::Cable,
            "7010" => Self::Rafter,
            "7020" => Self::Column,
            "8010" => Self::Railing,
            "8020" => Self::Stair,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Radiator => "1010",
            Self::Oven => "1020",
            Self::Fireside => "1030",
            Self::Ventilator => "1040",
            Self::AirConditioning => "1050",
            Self::Pipe => "5010",
            Self::Lamp => "3010",
            Self::LightSwitch => "3020",
            Self::PowerPoint => "5030",
            Self::Cable => "5020",
            Self::Rafter => "7010",
            Self::Column => "7020",
            Self::Railing => "8010",
            Self::Stair => "8020",
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
            ("1010", Sig3dIntBuildingInstallationFunctionValue::Radiator),
            ("8020", Sig3dIntBuildingInstallationFunctionValue::Stair),
        ] {
            assert_eq!(
                Sig3dIntBuildingInstallationFunctionValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dIntBuildingInstallationFunctionValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dIntBuildingInstallationFunctionValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }
}
