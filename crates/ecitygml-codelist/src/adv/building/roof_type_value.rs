use ecitygml_core::model::common::CodeListValue;

/// The AdV standard code list `RoofTypeTypeAdV.xml`, used to specify the
/// shape of a building's roof in German cadastral (ALKIS) and topographic
/// (ATKIS) CityGML datasets.
///
/// <https://repository.gdi-de.org/schemas/adv/citygml/Codelisten/RoofTypeTypeAdV.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdvRoofTypeTypeValue {
    /// Flachdach.
    FlatRoof,
    /// Pultdach.
    MonopitchRoof,
    /// Versetztes Pultdach.
    OffsetMonopitchRoof,
    /// Satteldach.
    GabledRoof,
    /// Walmdach.
    HippedRoof,
    /// Krüppelwalmdach.
    HalfHippedRoof,
    /// Mansardendach.
    MansardRoof,
    /// Zeltdach.
    PavilionRoof,
    /// Kegeldach.
    ConeRoof,
    /// Kuppeldach.
    DomeRoof,
    /// Sheddach.
    SawtoothRoof,
    /// Bogendach.
    ArchRoof,
    /// Turmdach.
    TowerRoof,
    /// Mischform.
    CombinationOfRoofForms,
    /// Sonstiges.
    OtherRoofForm,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for AdvRoofTypeTypeValue {
    const CODE_SPACE: &'static str =
        "https://repository.gdi-de.org/schemas/adv/citygml/Codelisten/RoofTypeTypeAdV.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1000" => Self::FlatRoof,
            "2100" => Self::MonopitchRoof,
            "2200" => Self::OffsetMonopitchRoof,
            "3100" => Self::GabledRoof,
            "3200" => Self::HippedRoof,
            "3300" => Self::HalfHippedRoof,
            "3400" => Self::MansardRoof,
            "3500" => Self::PavilionRoof,
            "3600" => Self::ConeRoof,
            "3700" => Self::DomeRoof,
            "3800" => Self::SawtoothRoof,
            "3900" => Self::ArchRoof,
            "4000" => Self::TowerRoof,
            "5000" => Self::CombinationOfRoofForms,
            "9999" => Self::OtherRoofForm,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::FlatRoof => "1000",
            Self::MonopitchRoof => "2100",
            Self::OffsetMonopitchRoof => "2200",
            Self::GabledRoof => "3100",
            Self::HippedRoof => "3200",
            Self::HalfHippedRoof => "3300",
            Self::MansardRoof => "3400",
            Self::PavilionRoof => "3500",
            Self::ConeRoof => "3600",
            Self::DomeRoof => "3700",
            Self::SawtoothRoof => "3800",
            Self::ArchRoof => "3900",
            Self::TowerRoof => "4000",
            Self::CombinationOfRoofForms => "5000",
            Self::OtherRoofForm => "9999",
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
            ("1000", AdvRoofTypeTypeValue::FlatRoof),
            ("3200", AdvRoofTypeTypeValue::HippedRoof),
            ("9999", AdvRoofTypeTypeValue::OtherRoofForm),
        ] {
            assert_eq!(AdvRoofTypeTypeValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = AdvRoofTypeTypeValue::from_code_value("777");
        assert_eq!(value, AdvRoofTypeTypeValue::Other("777".to_string()));
        assert_eq!(value.to_code_value(), "777");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = RoofTypeValue::from(Code::new("3200"));
        assert_eq!(
            wrapped.interpret::<AdvRoofTypeTypeValue>(),
            Some(AdvRoofTypeTypeValue::HippedRoof)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = RoofTypeValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1000",
        ));
        assert_eq!(wrapped.interpret::<AdvRoofTypeTypeValue>(), None);
    }
}
