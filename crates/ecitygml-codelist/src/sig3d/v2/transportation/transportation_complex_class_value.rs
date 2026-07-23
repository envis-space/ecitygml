use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `TransportationComplex_class.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/transportation/2.0/TransportationComplex_class.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dTransportationComplexClassValue {
    Private,
    Common,
    Civil,
    Military,
    RoadTraffic,
    AirTraffic,
    RailTraffic,
    Waterway,
    Subway,
    Others,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dTransportationComplexClassValue {
    const CODE_SPACE: &'static str = "http://www.sig3d.org/codelists/citygml/2.0/transportation/2.0/TransportationComplex_class.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::Private,
            "1010" => Self::Common,
            "1020" => Self::Civil,
            "1030" => Self::Military,
            "1040" => Self::RoadTraffic,
            "1050" => Self::AirTraffic,
            "1060" => Self::RailTraffic,
            "1070" => Self::Waterway,
            "1080" => Self::Subway,
            "1090" => Self::Others,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Private => "1000",
            Self::Common => "1010",
            Self::Civil => "1020",
            Self::Military => "1030",
            Self::RoadTraffic => "1040",
            Self::AirTraffic => "1050",
            Self::RailTraffic => "1060",
            Self::Waterway => "1070",
            Self::Subway => "1080",
            Self::Others => "1090",
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
            ("1000", Sig3dTransportationComplexClassValue::Private),
            ("1090", Sig3dTransportationComplexClassValue::Others),
        ] {
            assert_eq!(
                Sig3dTransportationComplexClassValue::from_code_value(code),
                value
            );
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dTransportationComplexClassValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dTransportationComplexClassValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }
}
