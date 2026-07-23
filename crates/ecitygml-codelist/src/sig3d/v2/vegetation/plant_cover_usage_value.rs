use ecitygml_core::model::common::CodeListValue;

/// The SIG3D standard code list `PlantCover_usage.xml` (version 2.0).
///
/// <https://www.sig3d.de/codelists/standard/vegetation/2.0/PlantCover_usage.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Sig3dPlantCoverUsageValue {
    Lemnetea,
    AsplenieteaRupestris,
    Adiantetea,
    ThlaspieteaRotundifolii,
    CrithmoLimonietea,
    Ammophietea,
    CakileteaMaritimaeHalophile,
    Secalinetea,
    Chenopodietea,
    Onopordetea,
    EpilobieteaAngustifolii,
    BidenteteaTripartiti,
    ZoostereteaMarinaeHalophile,
    RuppieteaMaritimae,
    PotameteaHaftende,
    Litorelletea,
    PlantagineteaMajoris,
    IsoetoNanojuncetea,
    MontinoCardaminetea,
    Corynephoretea,
    AstereteaTripolium,
    Salicornietea,
    JunceteaMaritimi,
    Phragmitetea,
    Spartinetea,
    SedoScleranthetea,
    SaliceteaHerbaceae,
    Arrhenatheretea,
    MolinioJuncetea,
    ScheuchzerioCariceteaFuscaeAzidophile,
    FestucoBrometea,
    ElynoSeslerietea,
    CariceteaCurvulaeAzidophile,
    CallunoUlicetea,
    OxycoccoSphagnetea,
    SaliceteaPurpureae,
    BetuloAdenostyletea,
    AlneteaGlutinosae,
    EricoPinetea,
    VaccinioPiceetea,
    QuerceteaRoboriPetraeae,
    QuercoFagetea,
    CrithmoStaticetea,
    TuberarieteaGuttati,
    JunceteaMaritimae,
    TheroBrachypodietea,
    OnonidoRosmarinetea,
    NerioTamaricetea,
    PeganoSalsoletea,
    CistoLavanduletea,
    QuerceteaIlicis,
    PopuleteaAlbae,
    Unknown,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for Sig3dPlantCoverUsageValue {
    const CODE_SPACE: &'static str =
        "http://www.sig3d.org/codelists/citygml/2.0/vegetation/2.0/PlantCover_usage.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "1010" => Self::Lemnetea,
            "1020" => Self::AsplenieteaRupestris,
            "1030" => Self::Adiantetea,
            "1040" => Self::ThlaspieteaRotundifolii,
            "1050" => Self::CrithmoLimonietea,
            "1060" => Self::Ammophietea,
            "1070" => Self::CakileteaMaritimaeHalophile,
            "1080" => Self::Secalinetea,
            "1090" => Self::Chenopodietea,
            "1100" => Self::Onopordetea,
            "1110" => Self::EpilobieteaAngustifolii,
            "1120" => Self::BidenteteaTripartiti,
            "1130" => Self::ZoostereteaMarinaeHalophile,
            "1140" => Self::RuppieteaMaritimae,
            "1150" => Self::PotameteaHaftende,
            "1160" => Self::Litorelletea,
            "1170" => Self::PlantagineteaMajoris,
            "1180" => Self::IsoetoNanojuncetea,
            "1190" => Self::MontinoCardaminetea,
            "1200" => Self::Corynephoretea,
            "1210" => Self::AstereteaTripolium,
            "1220" => Self::Salicornietea,
            "1230" => Self::JunceteaMaritimi,
            "1240" => Self::Phragmitetea,
            "1250" => Self::Spartinetea,
            "1260" => Self::SedoScleranthetea,
            "1270" => Self::SaliceteaHerbaceae,
            "1280" => Self::Arrhenatheretea,
            "1290" => Self::MolinioJuncetea,
            "1300" => Self::ScheuchzerioCariceteaFuscaeAzidophile,
            "1310" => Self::FestucoBrometea,
            "1320" => Self::ElynoSeslerietea,
            "1330" => Self::CariceteaCurvulaeAzidophile,
            "1340" => Self::CallunoUlicetea,
            "1350" => Self::OxycoccoSphagnetea,
            "1360" => Self::SaliceteaPurpureae,
            "1370" => Self::BetuloAdenostyletea,
            "1380" => Self::AlneteaGlutinosae,
            "1390" => Self::EricoPinetea,
            "1400" => Self::VaccinioPiceetea,
            "1410" => Self::QuerceteaRoboriPetraeae,
            "1420" => Self::QuercoFagetea,
            "1430" => Self::CrithmoStaticetea,
            "1440" => Self::TuberarieteaGuttati,
            "1450" => Self::JunceteaMaritimae,
            "1460" => Self::TheroBrachypodietea,
            "1470" => Self::OnonidoRosmarinetea,
            "1480" => Self::NerioTamaricetea,
            "1490" => Self::PeganoSalsoletea,
            "1500" => Self::CistoLavanduletea,
            "1510" => Self::QuerceteaIlicis,
            "1520" => Self::PopuleteaAlbae,
            "9999" => Self::Unknown,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::Lemnetea => "1010",
            Self::AsplenieteaRupestris => "1020",
            Self::Adiantetea => "1030",
            Self::ThlaspieteaRotundifolii => "1040",
            Self::CrithmoLimonietea => "1050",
            Self::Ammophietea => "1060",
            Self::CakileteaMaritimaeHalophile => "1070",
            Self::Secalinetea => "1080",
            Self::Chenopodietea => "1090",
            Self::Onopordetea => "1100",
            Self::EpilobieteaAngustifolii => "1110",
            Self::BidenteteaTripartiti => "1120",
            Self::ZoostereteaMarinaeHalophile => "1130",
            Self::RuppieteaMaritimae => "1140",
            Self::PotameteaHaftende => "1150",
            Self::Litorelletea => "1160",
            Self::PlantagineteaMajoris => "1170",
            Self::IsoetoNanojuncetea => "1180",
            Self::MontinoCardaminetea => "1190",
            Self::Corynephoretea => "1200",
            Self::AstereteaTripolium => "1210",
            Self::Salicornietea => "1220",
            Self::JunceteaMaritimi => "1230",
            Self::Phragmitetea => "1240",
            Self::Spartinetea => "1250",
            Self::SedoScleranthetea => "1260",
            Self::SaliceteaHerbaceae => "1270",
            Self::Arrhenatheretea => "1280",
            Self::MolinioJuncetea => "1290",
            Self::ScheuchzerioCariceteaFuscaeAzidophile => "1300",
            Self::FestucoBrometea => "1310",
            Self::ElynoSeslerietea => "1320",
            Self::CariceteaCurvulaeAzidophile => "1330",
            Self::CallunoUlicetea => "1340",
            Self::OxycoccoSphagnetea => "1350",
            Self::SaliceteaPurpureae => "1360",
            Self::BetuloAdenostyletea => "1370",
            Self::AlneteaGlutinosae => "1380",
            Self::EricoPinetea => "1390",
            Self::VaccinioPiceetea => "1400",
            Self::QuerceteaRoboriPetraeae => "1410",
            Self::QuercoFagetea => "1420",
            Self::CrithmoStaticetea => "1430",
            Self::TuberarieteaGuttati => "1440",
            Self::JunceteaMaritimae => "1450",
            Self::TheroBrachypodietea => "1460",
            Self::OnonidoRosmarinetea => "1470",
            Self::NerioTamaricetea => "1480",
            Self::PeganoSalsoletea => "1490",
            Self::CistoLavanduletea => "1500",
            Self::QuerceteaIlicis => "1510",
            Self::PopuleteaAlbae => "1520",
            Self::Unknown => "9999",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::vegetation::values::PlantCoverUsageValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            ("1010", Sig3dPlantCoverUsageValue::Lemnetea),
            ("9999", Sig3dPlantCoverUsageValue::Unknown),
        ] {
            assert_eq!(Sig3dPlantCoverUsageValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = Sig3dPlantCoverUsageValue::from_code_value("999999");
        assert_eq!(
            value,
            Sig3dPlantCoverUsageValue::Other("999999".to_string())
        );
        assert_eq!(value.to_code_value(), "999999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = PlantCoverUsageValue::from(Code::new("1010"));
        assert_eq!(
            wrapped.interpret::<Sig3dPlantCoverUsageValue>(),
            Some(Sig3dPlantCoverUsageValue::Lemnetea)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = PlantCoverUsageValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "1010",
        ));
        assert_eq!(wrapped.interpret::<Sig3dPlantCoverUsageValue>(), None);
    }
}
