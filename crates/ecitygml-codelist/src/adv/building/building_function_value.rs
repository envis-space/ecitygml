use ecitygml_core::model::common::CodeListValue;

/// The AdV standard code list `BuildingFunctionTypeAdV.xml`, used to specify
/// the function of a building in German cadastral (ALKIS) and topographic
/// (ATKIS) CityGML datasets.
///
/// <https://repository.gdi-de.org/schemas/adv/citygml/Codelisten/BuildingFunctionTypeAdV.xml>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdvBuildingFunctionTypeValue {
    /// Wohngebäude (ALKIS).
    ResidentialBuilding,
    /// Wohnhaus (ALKIS).
    ResidentialHouse,
    /// Wohnheim (ALKIS).
    ResidentialHome,
    /// Kinderheim (ALKIS).
    ChildrensHome,
    /// Seniorenheim (ALKIS).
    SeniorsHome,
    /// Schwesternwohnheim (ALKIS).
    NursesResidence,
    /// Studenten-, Schülerwohnheim (ALKIS).
    StudentOrPupilDormitory,
    /// Schullandheim (ALKIS).
    SchoolCampHouse,
    /// Gemischt genutztes Gebäude mit Wohnen (ALKIS).
    MixedUseBuildingWithResidential,
    /// Wohngebäude mit Gemeinbedarf (ALKIS).
    ResidentialBuildingWithCommunityFacilities,
    /// Wohngebäude mit Handel und Dienstleistungen (ALKIS).
    ResidentialBuildingWithRetailAndServices,
    /// Wohn- und Verwaltungsgebäude (ALKIS).
    ResidentialAndAdministrativeBuilding,
    /// Wohn- und Bürogebäude (ALKIS).
    ResidentialAndOfficeBuilding,
    /// Wohn- und Geschäftsgebäude (ALKIS).
    ResidentialAndCommercialBuilding,
    /// Wohngebäude mit Gewerbe und Industrie (ALKIS).
    ResidentialBuildingWithTradeAndIndustry,
    /// Wohn- und Betriebsgebäude (ALKIS).
    ResidentialAndOperationalBuilding,
    /// Land- und forstwirtschaftliches Wohngebäude (ALKIS).
    AgriculturalOrForestryResidentialBuilding,
    /// Land- und forstwirtschaftliches Wohn- und Betriebsgebäude (ALKIS).
    AgriculturalOrForestryResidentialAndOperationalBuilding,
    /// Bauernhaus (ALKIS).
    FarmHouse,
    /// Wohn- und Wirtschaftsgebäude (ALKIS).
    ResidentialAndFarmBuilding,
    /// Forsthaus (ALKIS).
    ForestersLodge,
    /// Gebäude zur Freizeitgestaltung (ALKIS).
    RecreationalBuilding,
    /// Ferienhaus (ALKIS).
    HolidayHouse,
    /// Wochenendhaus (ALKIS).
    WeekendHouse,
    /// Gartenhaus (ALKIS).
    GardenHouse,
    /// Gebäude für Wirtschaft oder Gewerbe (ALKIS).
    BuildingForEconomyOrTrade,
    /// Gebäude für Handel und Dienstleistungen (ALKIS).
    BuildingForRetailAndServices,
    /// Bürogebäude (ALKIS).
    OfficeBuilding,
    /// Kreditinstitut (ALKIS).
    CreditInstitution,
    /// Versicherung (ALKIS).
    InsuranceCompany,
    /// Geschäftsgebäude (ALKIS).
    CommercialBuilding,
    /// Kaufhaus (ALKIS).
    DepartmentStore,
    /// Einkaufszentrum (ALKIS).
    ShoppingCentre,
    /// Markthalle (ALKIS).
    MarketHall,
    /// Laden (ALKIS).
    Shop,
    /// Kiosk (ALKIS).
    Kiosk,
    /// Apotheke (ALKIS).
    Pharmacy,
    /// Messehalle (ALKIS).
    ExhibitionHall,
    /// Gebäude für Beherbergung (ALKIS).
    BuildingForAccommodation,
    /// Hotel, Motel, Pension (ALKIS).
    HotelMotelOrGuesthouse,
    /// Jugendherberge (ALKIS).
    YouthHostel,
    /// Hütte (mit Übernachtungsmöglichkeit) (ALKIS).
    CabinWithOvernightAccommodation,
    /// Campingplatzgebäude (ALKIS).
    CampsiteBuilding,
    /// Gebäude für Bewirtung (ALKIS).
    BuildingForCatering,
    /// Gaststätte, Restaurant (ALKIS).
    RestaurantOrInn,
    /// Hütte (ohne Übernachtungsmöglichkeit) (ALKIS).
    CabinWithoutOvernightAccommodation,
    /// Kantine (ALKIS).
    Canteen,
    /// Freizeit- und Vergnügungsstätte (ALKIS).
    RecreationAndEntertainmentVenue,
    /// Festsaal (ALKIS).
    BanquetHall,
    /// Kino (ALKIS).
    Cinema,
    /// Kegel-, Bowlinghalle (ALKIS).
    BowlingAlley,
    /// Spielkasino (ALKIS).
    Casino,
    /// Spielhalle (ALKIS).
    AmusementArcade,
    /// Gebäude für Gewerbe und Industrie (ALKIS).
    BuildingForTradeAndIndustry,
    /// Produktionsgebäude (ALKIS).
    ProductionBuilding,
    /// Fabrik (ALKIS).
    Factory,
    /// Betriebsgebäude (ALKIS).
    OperationalBuilding,
    /// Brauerei (ALKIS).
    Brewery,
    /// Brennerei (ALKIS).
    Distillery,
    /// Werkstatt (ALKIS).
    Workshop,
    /// Sägewerk (ALKIS).
    Sawmill,
    /// Tankstelle (ALKIS).
    PetrolStation,
    /// Waschstraße, Waschanlage, Waschhalle (ALKIS).
    CarWashFacility,
    /// Gebäude für Vorratshaltung (ALKIS).
    BuildingForStorage,
    /// Kühlhaus (ALKIS).
    ColdStore,
    /// Speichergebäude (ALKIS).
    WarehouseBuilding,
    /// Lagerhalle, Lagerschuppen, Lagerhaus (ALKIS).
    StorageHallOrShed,
    /// Speditionsgebäude (ALKIS).
    FreightForwardingBuilding,
    /// Gebäude für Forschungszwecke (ALKIS).
    BuildingForResearchPurposes,
    /// Gebäude für Grundstoffgewinnung (ALKIS).
    BuildingForRawMaterialExtraction,
    /// Bergwerk (ALKIS).
    Mine,
    /// Saline (ALKIS).
    SaltWorks,
    /// Gebäude für betriebliche Sozialeinrichtung (ALKIS).
    BuildingForCompanyWelfareFacility,
    /// Sonstiges Gebäude für Gewerbe und Industrie (ALKIS).
    OtherBuildingForTradeAndIndustry,
    /// Mühle (ALKIS).
    Mill,
    /// Windmühle (ALKIS).
    Windmill,
    /// Wassermühle (ALKIS).
    WaterMill,
    /// Schöpfwerk (ALKIS).
    WaterScoopMill,
    /// Wetterstation (ALKIS).
    WeatherStation,
    /// Gebäude für Handel und Dienstleistung mit Wohnen (ALKIS).
    BuildingForRetailAndServicesWithResidential,
    /// Gebäude für Gewerbe und Industrie mit Wohnen (ALKIS).
    BuildingForTradeAndIndustryWithResidential,
    /// Betriebsgebäude zu Verkehrsanlagen (allgemein) (ALKIS).
    OperationalBuildingForTransportFacilitiesGeneral,
    /// Betriebsgebäude für Straßenverkehr (ALKIS).
    OperationalBuildingForRoadTraffic,
    /// Straßenmeisterei (ALKIS).
    RoadMaintenanceDepot,
    /// Wartehalle (ALKIS).
    WaitingHall,
    /// Betriebsgebäude für Schienenverkehr (ALKIS).
    OperationalBuildingForRailTraffic,
    /// Bahnwärterhaus (ALKIS).
    RailwayWatchmansHouse,
    /// Lokschuppen, Wagenhalle (ALKIS).
    EngineShedOrCarriageHall,
    /// Stellwerk, Blockstelle (ALKIS).
    SignalBoxOrBlockPost,
    /// Betriebsgebäude des Güterbahnhofs (ALKIS).
    FreightStationOperationalBuilding,
    /// Betriebsgebäude für Flugverkehr (ALKIS).
    OperationalBuildingForAirTraffic,
    /// Flugzeughalle (ALKIS).
    AircraftHangar,
    /// Betriebsgebäude für Schiffsverkehr (ALKIS).
    OperationalBuildingForShipTraffic,
    /// Werft (Halle) (ALKIS).
    ShipyardHall,
    /// Dock (Halle) (ALKIS).
    DockHall,
    /// Betriebsgebäude zur Schleuse (ALKIS).
    LockOperationalBuilding,
    /// Bootshaus (ALKIS).
    Boathouse,
    /// Betriebsgebäude zur Seilbahn (ALKIS).
    CablewayOperationalBuilding,
    /// Spannwerk zur Drahtseilbahn (ALKIS).
    CablewayTensioningStation,
    /// Gebäude zum Parken (ALKIS).
    BuildingForParking,
    /// Parkhaus (ALKIS).
    MultiStoreyCarPark,
    /// Parkdeck (ALKIS).
    ParkingDeck,
    /// Garage (ALKIS).
    Garage,
    /// Fahrzeughalle (ALKIS).
    VehicleHall,
    /// Gebäude zur Versorgung (ALKIS).
    BuildingForUtilitySupply,
    /// Gebäude zur Energieversorgung (ALKIS).
    BuildingForEnergySupply,
    /// Gebäude zur Wasserversorgung (ALKIS).
    BuildingForWaterSupply,
    /// Wasserwerk (ALKIS).
    Waterworks,
    /// Pumpstation (ALKIS).
    PumpStation,
    /// Wasserbehälter (ALKIS).
    WaterTank,
    /// Gebäude zur Elektrizitätsversorgung (ALKIS).
    BuildingForElectricitySupply,
    /// Elektrizitätswerk (ALKIS).
    PowerPlant,
    /// Umspannwerk (ALKIS).
    TransformerStation,
    /// Umformer (ALKIS).
    ElectricalConverterBuilding,
    /// Reaktorgebäude (ALKIS).
    ReactorBuilding,
    /// Turbinenhaus (ALKIS).
    TurbineHouse,
    /// Kesselhaus (ALKIS).
    BoilerHouse,
    /// Gebäude für Fernmeldewesen (ALKIS).
    BuildingForTelecommunications,
    /// Gebäude an unterirdischen Leitungen (ALKIS).
    BuildingAtUndergroundPipelines,
    /// Gebäude zur Gasversorgung (ALKIS).
    BuildingForGasSupply,
    /// Gaswerk (ALKIS).
    GasWorks,
    /// Heizwerk (ALKIS).
    HeatingPlant,
    /// Gebäude zur Versorgungsanlage (ALKIS).
    BuildingForSupplyFacility,
    /// Pumpwerk (nicht für Wasserversorgung) (ALKIS).
    PumpingPlantNotForWaterSupply,
    /// Gebäude zur Entsorgung (ALKIS).
    BuildingForWasteDisposal,
    /// Gebäude zur Abwasserbeseitigung (ALKIS).
    BuildingForWastewaterDisposal,
    /// Gebäude der Kläranlage (ALKIS).
    SewageTreatmentPlantBuilding,
    /// Toilette (ALKIS).
    Toilet,
    /// Gebäude zur Abfallbehandlung (ALKIS).
    BuildingForWasteTreatment,
    /// Müllbunker (ALKIS).
    RefuseBunker,
    /// Gebäude zur Müllverbrennung (ALKIS).
    WasteIncinerationBuilding,
    /// Gebäude der Abfalldeponie (ALKIS).
    LandfillBuilding,
    /// Gebäude für Land- und Forstwirtschaft (ALKIS).
    BuildingForAgricultureAndForestry,
    /// Land- und forstwirtschaftliches Betriebsgebäude (ALKIS).
    AgriculturalOrForestryOperationalBuilding,
    /// Scheune (ALKIS).
    Barn,
    /// Schuppen (ALKIS).
    Shed,
    /// Stall (ALKIS).
    Stable,
    /// Scheune und Stall (ALKIS).
    BarnAndStable,
    /// Stall für Tiergroßhaltung (ALKIS).
    IntensiveLivestockStable,
    /// Reithalle (ALKIS).
    RidingHall,
    /// Wirtschaftsgebäude (ALKIS).
    FarmOutbuilding,
    /// Almhütte (ALKIS).
    AlpineHut,
    /// Jagdhaus, Jagdhütte (ALKIS).
    HuntingLodge,
    /// Treibhaus, Gewächshaus (ALKIS).
    Greenhouse,
    /// Treibhaus (ALKIS).
    Hothouse,
    /// Gewächshaus, verschiebbar (ALKIS).
    MovableGreenhouse,
    /// Gebäude für öffentliche Zwecke (ALKIS).
    BuildingForPublicPurposes,
    /// Verwaltungsgebäude (ALKIS).
    AdministrativeBuilding,
    /// Parlament (ALKIS).
    Parliament,
    /// Rathaus (ALKIS).
    CityHall,
    /// Post (ALKIS).
    PostOffice,
    /// Zollamt (ALKIS).
    CustomsOffice,
    /// Gericht (ALKIS).
    CourtOfLaw,
    /// Botschaft, Konsulat (ALKIS).
    EmbassyOrConsulate,
    /// Kreisverwaltung (ALKIS).
    DistrictAdministration,
    /// Bezirksregierung (ALKIS).
    RegionalGovernment,
    /// Finanzamt (ALKIS).
    TaxOffice,
    /// Gebäude für Bildung und Forschung (ALKIS).
    BuildingForEducationAndResearch,
    /// Allgemein bildende Schule (ALKIS).
    GeneralEducationSchool,
    /// Berufsbildende Schule (ALKIS).
    VocationalSchool,
    /// Hochschulgebäude (Fachhochschule, Universität) (ALKIS).
    UniversityBuilding,
    /// Forschungsinstitut (ALKIS).
    ResearchInstitute,
    /// Gebäude für kulturelle Zwecke (ALKIS).
    BuildingForCulturalPurposes,
    /// Schloss (ALKIS).
    Palace,
    /// Theater, Oper (ALKIS).
    TheatreOrOpera,
    /// Konzertgebäude (ALKIS).
    ConcertHall,
    /// Museum (ALKIS).
    Museum,
    /// Rundfunk, Fernsehen (ALKIS).
    BroadcastingBuilding,
    /// Veranstaltungsgebäude (ALKIS).
    EventBuilding,
    /// Bibliothek, Bücherei (ALKIS).
    Library,
    /// Burg, Festung (ALKIS).
    CastleOrFortress,
    /// Gebäude für religiöse Zwecke (ALKIS).
    BuildingForReligiousPurposes,
    /// Kirche (ALKIS).
    Church,
    /// Synagoge (ALKIS).
    Synagogue,
    /// Kapelle (ALKIS).
    Chapel,
    /// Gemeindehaus (ALKIS).
    ParishHall,
    /// Gotteshaus (ALKIS).
    HouseOfWorship,
    /// Moschee (ALKIS).
    Mosque,
    /// Tempel (ALKIS).
    Temple,
    /// Kloster (ALKIS).
    Monastery,
    /// Gebäude für Gesundheitswesen (ALKIS).
    BuildingForHealthCare,
    /// Krankenhaus (ALKIS).
    Hospital,
    /// Heilanstalt, Pflegeanstalt, Pflegestation (ALKIS).
    NursingOrCareFacility,
    /// Ärztehaus, Poliklinik (ALKIS).
    MedicalCentreOrPolyclinic,
    /// Rettungswache (ALKIS).
    AmbulanceStation,
    /// Gebäude für soziale Zwecke (ALKIS).
    BuildingForSocialPurposes,
    /// Jugendfreizeitheim (ALKIS).
    YouthCentre,
    /// Freizeit-, Vereinsheim, Dorfgemeinschafts-, Bürgerhaus (ALKIS).
    CommunityOrClubHouse,
    /// Seniorenfreizeitstätte (ALKIS).
    SeniorsRecreationCentre,
    /// Obdachlosenheim (ALKIS).
    HomelessShelter,
    /// Kinderkrippe, Kindergarten, Kindertagesstätte (ALKIS).
    NurseryOrKindergarten,
    /// Asylbewerberheim (ALKIS).
    AsylumSeekersHome,
    /// Gebäude für Sicherheit und Ordnung (ALKIS).
    BuildingForPublicSafetyAndOrder,
    /// Polizei (ALKIS).
    PoliceStation,
    /// Feuerwehr (ALKIS).
    FireStation,
    /// Kaserne (ALKIS).
    Barracks,
    /// Schutzbunker (ALKIS).
    Bunker,
    /// Justizvollzugsanstalt (ALKIS).
    Prison,
    /// Friedhofsgebäude (ALKIS).
    CemeteryBuilding,
    /// Trauerhalle (ALKIS).
    FuneralHall,
    /// Krematorium (ALKIS).
    Crematorium,
    /// Empfangsgebäude (ALKIS).
    ReceptionBuilding,
    /// Bahnhofsgebäude (ALKIS).
    RailwayStationBuilding,
    /// Flughafengebäude (ALKIS).
    AirportBuilding,
    /// Gebäude zum U-Bahnhof (ALKIS).
    UndergroundStationBuilding,
    /// Gebäude zum S-Bahnhof (ALKIS).
    SuburbanRailStationBuilding,
    /// Gebäude zum Busbahnhof (ALKIS).
    BusStationBuilding,
    /// Empfangsgebäude Schifffahrt (ALKIS).
    ShippingTerminalBuilding,
    /// Gebäude für öffentliche Zwecke mit Wohnen (ALKIS).
    BuildingForPublicPurposesWithResidential,
    /// Gebäude für Erholungszwecke (ALKIS).
    BuildingForRecreationalPurposes,
    /// Gebäude für Sportzwecke (ALKIS).
    BuildingForSportsPurposes,
    /// Sport-, Turnhalle (ALKIS).
    SportsOrGymHall,
    /// Gebäude zum Sportplatz (ALKIS).
    SportsGroundBuilding,
    /// Badegebäude (ALKIS).
    BathingBuilding,
    /// Hallenbad (ALKIS).
    IndoorSwimmingPool,
    /// Gebäude im Freibad (ALKIS).
    OutdoorPoolBuilding,
    /// Gebäude im Stadion (ALKIS).
    StadiumBuilding,
    /// Gebäude für Kurbetrieb (ALKIS).
    BuildingForSpaOperations,
    /// Badegebäude für medizinische Zwecke (ALKIS).
    MedicalBathingBuilding,
    /// Sanatorium (ALKIS).
    Sanatorium,
    /// Gebäude im Zoo (ALKIS).
    ZooBuilding,
    /// Empfangsgebäude des Zoos (ALKIS).
    ZooReceptionBuilding,
    /// Aquarium, Terrarium, Voliere (ALKIS).
    AquariumTerrariumOrAviary,
    /// Tierschauhaus (ALKIS).
    AnimalShowHouse,
    /// Stall im Zoo (ALKIS).
    ZooStable,
    /// Gebäude im botanischen Garten (ALKIS).
    BotanicalGardenBuilding,
    /// Empfangsgebäude des botanischen Gartens (ALKIS).
    BotanicalGardenReceptionBuilding,
    /// Gewächshaus (Botanik) (ALKIS).
    BotanicalGreenhouse,
    /// Pflanzenschauhaus (ALKIS).
    PlantShowHouse,
    /// Gebäude für andere Erholungseinrichtung (ALKIS).
    BuildingForOtherRecreationalFacility,
    /// Schutzhütte (ALKIS).
    ShelterHut,
    /// Touristisches Informationszentrum (ALKIS).
    TouristInformationCentre,
    /// Nach Quellenlage nicht zu spezifizieren (ALKIS).
    NotSpecifiableFromSourceAlkis,
    /// Wasserturm (ALKIS-ATKIS).
    WaterTower,
    /// Kirchturm (ALKIS-ATKIS).
    ChurchTower,
    /// Aussichtsturm (ALKIS-ATKIS).
    ObservationTower,
    /// Kontrollturm (ALKIS-ATKIS).
    ControlTower,
    /// Kühlturm (ALKIS-ATKIS).
    CoolingTower,
    /// Leuchtturm (ALKIS-ATKIS).
    Lighthouse,
    /// Feuerwachturm (ALKIS-ATKIS).
    FireWatchTower,
    /// Sende-,Funkturm (ALKIS-ATKIS).
    BroadcastingOrRadioTower,
    /// Stadt-,Torturm (ALKIS-ATKIS).
    CityOrGateTower,
    /// Förderturm (ALKIS-ATKIS).
    MineHeadframe,
    /// Bohrturm (ALKIS).
    DrillingRig,
    /// Schloss-, Burgturm (ALKIS-ATKIS).
    PalaceOrCastleTower,
    /// Nach Quellenlage nicht zu spezifizieren (ALKIS-ATKIS).
    NotSpecifiableFromSourceAlkisAtkis,
    /// Sonstiges (ALKIS-ATKIS).
    OtherTower,
    /// Biogasanlage (ALKIS).
    BiogasPlant,
    /// Windrad (ALKIS-ATKIS).
    WindTurbine,
    /// Solarzellen (ALKIS).
    SolarPanelInstallation,
    /// Mast (ALKIS).
    Mast,
    /// Freileitungsmast (ATKIS).
    OverheadPowerLinePylon,
    /// Funkmast (ALKIS-ATKIS).
    RadioMast,
    /// Radioteleskop (ALKIS-ATKIS).
    RadioTelescope,
    /// Schornstein (ALKIS-ATKIS).
    Chimney,
    /// Kran (ALKIS).
    Crane,
    /// Drehkran (ALKIS).
    SlewingCrane,
    /// Portalkran (ALKIS).
    GantryCrane,
    /// Laufkran (ALKIS).
    OverheadTravellingCrane,
    /// Hochofen (ALKIS).
    BlastFurnace,
    /// Umformer (ALKIS).
    RotaryConverterBuilding,
    /// Sonstiges (ALKIS).
    OtherIndustrialStructure,
    /// Silo (ALKIS).
    Silo,
    /// Tank (ALKIS).
    Tank,
    /// Gasometer (ALKIS).
    Gasometer,
    /// Sonstiges (ALKIS).
    OtherContainerStructure,
    /// Zuschauertribüne (ALKIS-ATKIS).
    Grandstand,
    /// Zuschauertribüne, überdacht (ALKIS).
    CoveredGrandstand,
    /// Zuschauertribüne, nicht überdacht (ALKIS).
    UncoveredGrandstand,
    /// Stadion (ALKIS-ATKIS).
    Stadium,
    /// Stadion, überdacht (ALKIS-ATKIS).
    CoveredStadium,
    /// Stadion, nicht überdacht (ALKIS-ATKIS).
    UncoveredStadium,
    /// Sprungschanze (Anlauf) (ALKIS-ATKIS).
    SkiJumpInrun,
    /// Gradierwerk (ALKIS-ATKIS).
    GraduationTower,
    /// Sonstiges (ALKIS).
    OtherSportsOrSpaStructure,
    /// Aquädukt (ATKIS).
    Aqueduct,
    /// Wachturm (ALKIS-ATKIS).
    WatchTower,
    /// Befestigung (Burgruine) (ALKIS).
    Fortification,
    /// Historische Mauer (ALKIS-ATKIS).
    HistoricWall,
    /// Stadtmauer (ALKIS).
    CityWall,
    /// Sonstige historische Mauer (ALKIS).
    OtherHistoricWall,
    /// Sonstiges (ALKIS).
    OtherHistoricStructure,
    /// Überdachung (ALKIS).
    Canopy,
    /// Carport (ALKIS).
    Carport,
    /// Mauer (ATKIS).
    Wall,
    /// Denkmal (ALKIS).
    Monument,
    /// Sonstiges (ALKIS).
    OtherMiscellaneousStructure,
    /// Schiffshebewerk (ATKIS).
    BoatLift,
    /// Kammerschleuse (ATKIS).
    ChamberLock,
    /// Brücke (ATKIS).
    Bridge,
    /// Drehbrücke (ATKIS).
    SwingBridge,
    /// Hebebrücke (ATKIS).
    LiftBridge,
    /// Zugbrücke (ATKIS).
    Drawbridge,
    /// Hochbahn, Hochstraße (ATKIS).
    ElevatedRailwayOrRoad,
    /// Schleusenkammer (ATKIS).
    LockChamber,
    /// Staumauer (ATKIS).
    DamWall,
    /// Wehr (ATKIS).
    Weir,
    /// Sicherheitstor (ATKIS).
    SafetyGate,
    /// Siel (ATKIS).
    Sluice,
    /// Sperrwerk (ATKIS).
    BarrageOrStormSurgeBarrier,
    /// Schöpfwerk (ATKIS).
    DrainagePumpingStation,
    /// A value within this code list's codeSpace that isn't one of the
    /// entries above.
    Other(String),
}

impl CodeListValue for AdvBuildingFunctionTypeValue {
    const CODE_SPACE: &'static str =
        "https://repository.gdi-de.org/schemas/adv/citygml/Codelisten/BuildingFunctionTypeAdV.xml";

    fn from_code_value(value: &str) -> Self {
        match value {
            "31001_1000" => Self::ResidentialBuilding,
            "31001_1010" => Self::ResidentialHouse,
            "31001_1020" => Self::ResidentialHome,
            "31001_1021" => Self::ChildrensHome,
            "31001_1022" => Self::SeniorsHome,
            "31001_1023" => Self::NursesResidence,
            "31001_1024" => Self::StudentOrPupilDormitory,
            "31001_1025" => Self::SchoolCampHouse,
            "31001_1100" => Self::MixedUseBuildingWithResidential,
            "31001_1110" => Self::ResidentialBuildingWithCommunityFacilities,
            "31001_1120" => Self::ResidentialBuildingWithRetailAndServices,
            "31001_1121" => Self::ResidentialAndAdministrativeBuilding,
            "31001_1122" => Self::ResidentialAndOfficeBuilding,
            "31001_1123" => Self::ResidentialAndCommercialBuilding,
            "31001_1130" => Self::ResidentialBuildingWithTradeAndIndustry,
            "31001_1131" => Self::ResidentialAndOperationalBuilding,
            "31001_1210" => Self::AgriculturalOrForestryResidentialBuilding,
            "31001_1220" => Self::AgriculturalOrForestryResidentialAndOperationalBuilding,
            "31001_1221" => Self::FarmHouse,
            "31001_1222" => Self::ResidentialAndFarmBuilding,
            "31001_1223" => Self::ForestersLodge,
            "31001_1310" => Self::RecreationalBuilding,
            "31001_1311" => Self::HolidayHouse,
            "31001_1312" => Self::WeekendHouse,
            "31001_1313" => Self::GardenHouse,
            "31001_2000" => Self::BuildingForEconomyOrTrade,
            "31001_2010" => Self::BuildingForRetailAndServices,
            "31001_2020" => Self::OfficeBuilding,
            "31001_2030" => Self::CreditInstitution,
            "31001_2040" => Self::InsuranceCompany,
            "31001_2050" => Self::CommercialBuilding,
            "31001_2051" => Self::DepartmentStore,
            "31001_2052" => Self::ShoppingCentre,
            "31001_2053" => Self::MarketHall,
            "31001_2054" => Self::Shop,
            "31001_2055" => Self::Kiosk,
            "31001_2056" => Self::Pharmacy,
            "31001_2060" => Self::ExhibitionHall,
            "31001_2070" => Self::BuildingForAccommodation,
            "31001_2071" => Self::HotelMotelOrGuesthouse,
            "31001_2072" => Self::YouthHostel,
            "31001_2073" => Self::CabinWithOvernightAccommodation,
            "31001_2074" => Self::CampsiteBuilding,
            "31001_2080" => Self::BuildingForCatering,
            "31001_2081" => Self::RestaurantOrInn,
            "31001_2082" => Self::CabinWithoutOvernightAccommodation,
            "31001_2083" => Self::Canteen,
            "31001_2090" => Self::RecreationAndEntertainmentVenue,
            "31001_2091" => Self::BanquetHall,
            "31001_2092" => Self::Cinema,
            "31001_2093" => Self::BowlingAlley,
            "31001_2094" => Self::Casino,
            "31001_2095" => Self::AmusementArcade,
            "31001_2100" => Self::BuildingForTradeAndIndustry,
            "31001_2110" => Self::ProductionBuilding,
            "31001_2111" => Self::Factory,
            "31001_2112" => Self::OperationalBuilding,
            "31001_2113" => Self::Brewery,
            "31001_2114" => Self::Distillery,
            "31001_2120" => Self::Workshop,
            "31001_2121" => Self::Sawmill,
            "31001_2130" => Self::PetrolStation,
            "31001_2131" => Self::CarWashFacility,
            "31001_2140" => Self::BuildingForStorage,
            "31001_2141" => Self::ColdStore,
            "31001_2142" => Self::WarehouseBuilding,
            "31001_2143" => Self::StorageHallOrShed,
            "31001_2150" => Self::FreightForwardingBuilding,
            "31001_2160" => Self::BuildingForResearchPurposes,
            "31001_2170" => Self::BuildingForRawMaterialExtraction,
            "31001_2171" => Self::Mine,
            "31001_2172" => Self::SaltWorks,
            "31001_2180" => Self::BuildingForCompanyWelfareFacility,
            "31001_2200" => Self::OtherBuildingForTradeAndIndustry,
            "31001_2210" => Self::Mill,
            "31001_2211" => Self::Windmill,
            "31001_2212" => Self::WaterMill,
            "31001_2213" => Self::WaterScoopMill,
            "31001_2220" => Self::WeatherStation,
            "31001_2310" => Self::BuildingForRetailAndServicesWithResidential,
            "31001_2320" => Self::BuildingForTradeAndIndustryWithResidential,
            "31001_2400" => Self::OperationalBuildingForTransportFacilitiesGeneral,
            "31001_2410" => Self::OperationalBuildingForRoadTraffic,
            "31001_2411" => Self::RoadMaintenanceDepot,
            "31001_2412" => Self::WaitingHall,
            "31001_2420" => Self::OperationalBuildingForRailTraffic,
            "31001_2421" => Self::RailwayWatchmansHouse,
            "31001_2422" => Self::EngineShedOrCarriageHall,
            "31001_2423" => Self::SignalBoxOrBlockPost,
            "31001_2424" => Self::FreightStationOperationalBuilding,
            "31001_2430" => Self::OperationalBuildingForAirTraffic,
            "31001_2431" => Self::AircraftHangar,
            "31001_2440" => Self::OperationalBuildingForShipTraffic,
            "31001_2441" => Self::ShipyardHall,
            "31001_2442" => Self::DockHall,
            "31001_2443" => Self::LockOperationalBuilding,
            "31001_2444" => Self::Boathouse,
            "31001_2450" => Self::CablewayOperationalBuilding,
            "31001_2451" => Self::CablewayTensioningStation,
            "31001_2460" => Self::BuildingForParking,
            "31001_2461" => Self::MultiStoreyCarPark,
            "31001_2462" => Self::ParkingDeck,
            "31001_2463" => Self::Garage,
            "31001_2464" => Self::VehicleHall,
            "31001_2500" => Self::BuildingForUtilitySupply,
            "31001_2501" => Self::BuildingForEnergySupply,
            "31001_2510" => Self::BuildingForWaterSupply,
            "31001_2511" => Self::Waterworks,
            "31001_2512" => Self::PumpStation,
            "31001_2513" => Self::WaterTank,
            "31001_2520" => Self::BuildingForElectricitySupply,
            "31001_2521" => Self::PowerPlant,
            "31001_2522" => Self::TransformerStation,
            "31001_2523" => Self::ElectricalConverterBuilding,
            "31001_2527" => Self::ReactorBuilding,
            "31001_2528" => Self::TurbineHouse,
            "31001_2529" => Self::BoilerHouse,
            "31001_2540" => Self::BuildingForTelecommunications,
            "31001_2560" => Self::BuildingAtUndergroundPipelines,
            "31001_2570" => Self::BuildingForGasSupply,
            "31001_2571" => Self::GasWorks,
            "31001_2580" => Self::HeatingPlant,
            "31001_2590" => Self::BuildingForSupplyFacility,
            "31001_2591" => Self::PumpingPlantNotForWaterSupply,
            "31001_2600" => Self::BuildingForWasteDisposal,
            "31001_2610" => Self::BuildingForWastewaterDisposal,
            "31001_2611" => Self::SewageTreatmentPlantBuilding,
            "31001_2612" => Self::Toilet,
            "31001_2620" => Self::BuildingForWasteTreatment,
            "31001_2621" => Self::RefuseBunker,
            "31001_2622" => Self::WasteIncinerationBuilding,
            "31001_2623" => Self::LandfillBuilding,
            "31001_2700" => Self::BuildingForAgricultureAndForestry,
            "31001_2720" => Self::AgriculturalOrForestryOperationalBuilding,
            "31001_2721" => Self::Barn,
            "31001_2723" => Self::Shed,
            "31001_2724" => Self::Stable,
            "31001_2726" => Self::BarnAndStable,
            "31001_2727" => Self::IntensiveLivestockStable,
            "31001_2728" => Self::RidingHall,
            "31001_2729" => Self::FarmOutbuilding,
            "31001_2732" => Self::AlpineHut,
            "31001_2735" => Self::HuntingLodge,
            "31001_2740" => Self::Greenhouse,
            "31001_2741" => Self::Hothouse,
            "31001_2742" => Self::MovableGreenhouse,
            "31001_3000" => Self::BuildingForPublicPurposes,
            "31001_3010" => Self::AdministrativeBuilding,
            "31001_3011" => Self::Parliament,
            "31001_3012" => Self::CityHall,
            "31001_3013" => Self::PostOffice,
            "31001_3014" => Self::CustomsOffice,
            "31001_3015" => Self::CourtOfLaw,
            "31001_3016" => Self::EmbassyOrConsulate,
            "31001_3017" => Self::DistrictAdministration,
            "31001_3018" => Self::RegionalGovernment,
            "31001_3019" => Self::TaxOffice,
            "31001_3020" => Self::BuildingForEducationAndResearch,
            "31001_3021" => Self::GeneralEducationSchool,
            "31001_3022" => Self::VocationalSchool,
            "31001_3023" => Self::UniversityBuilding,
            "31001_3024" => Self::ResearchInstitute,
            "31001_3030" => Self::BuildingForCulturalPurposes,
            "31001_3031" => Self::Palace,
            "31001_3032" => Self::TheatreOrOpera,
            "31001_3033" => Self::ConcertHall,
            "31001_3034" => Self::Museum,
            "31001_3035" => Self::BroadcastingBuilding,
            "31001_3036" => Self::EventBuilding,
            "31001_3037" => Self::Library,
            "31001_3038" => Self::CastleOrFortress,
            "31001_3040" => Self::BuildingForReligiousPurposes,
            "31001_3041" => Self::Church,
            "31001_3042" => Self::Synagogue,
            "31001_3043" => Self::Chapel,
            "31001_3044" => Self::ParishHall,
            "31001_3045" => Self::HouseOfWorship,
            "31001_3046" => Self::Mosque,
            "31001_3047" => Self::Temple,
            "31001_3048" => Self::Monastery,
            "31001_3050" => Self::BuildingForHealthCare,
            "31001_3051" => Self::Hospital,
            "31001_3052" => Self::NursingOrCareFacility,
            "31001_3053" => Self::MedicalCentreOrPolyclinic,
            "31001_3054" => Self::AmbulanceStation,
            "31001_3060" => Self::BuildingForSocialPurposes,
            "31001_3061" => Self::YouthCentre,
            "31001_3062" => Self::CommunityOrClubHouse,
            "31001_3063" => Self::SeniorsRecreationCentre,
            "31001_3064" => Self::HomelessShelter,
            "31001_3065" => Self::NurseryOrKindergarten,
            "31001_3066" => Self::AsylumSeekersHome,
            "31001_3070" => Self::BuildingForPublicSafetyAndOrder,
            "31001_3071" => Self::PoliceStation,
            "31001_3072" => Self::FireStation,
            "31001_3073" => Self::Barracks,
            "31001_3074" => Self::Bunker,
            "31001_3075" => Self::Prison,
            "31001_3080" => Self::CemeteryBuilding,
            "31001_3081" => Self::FuneralHall,
            "31001_3082" => Self::Crematorium,
            "31001_3090" => Self::ReceptionBuilding,
            "31001_3091" => Self::RailwayStationBuilding,
            "31001_3092" => Self::AirportBuilding,
            "31001_3094" => Self::UndergroundStationBuilding,
            "31001_3095" => Self::SuburbanRailStationBuilding,
            "31001_3097" => Self::BusStationBuilding,
            "31001_3098" => Self::ShippingTerminalBuilding,
            "31001_3100" => Self::BuildingForPublicPurposesWithResidential,
            "31001_3200" => Self::BuildingForRecreationalPurposes,
            "31001_3210" => Self::BuildingForSportsPurposes,
            "31001_3211" => Self::SportsOrGymHall,
            "31001_3212" => Self::SportsGroundBuilding,
            "31001_3220" => Self::BathingBuilding,
            "31001_3221" => Self::IndoorSwimmingPool,
            "31001_3222" => Self::OutdoorPoolBuilding,
            "31001_3230" => Self::StadiumBuilding,
            "31001_3240" => Self::BuildingForSpaOperations,
            "31001_3241" => Self::MedicalBathingBuilding,
            "31001_3242" => Self::Sanatorium,
            "31001_3260" => Self::ZooBuilding,
            "31001_3261" => Self::ZooReceptionBuilding,
            "31001_3262" => Self::AquariumTerrariumOrAviary,
            "31001_3263" => Self::AnimalShowHouse,
            "31001_3264" => Self::ZooStable,
            "31001_3270" => Self::BotanicalGardenBuilding,
            "31001_3271" => Self::BotanicalGardenReceptionBuilding,
            "31001_3272" => Self::BotanicalGreenhouse,
            "31001_3273" => Self::PlantShowHouse,
            "31001_3280" => Self::BuildingForOtherRecreationalFacility,
            "31001_3281" => Self::ShelterHut,
            "31001_3290" => Self::TouristInformationCentre,
            "31001_9998" => Self::NotSpecifiableFromSourceAlkis,
            "51001_1001" => Self::WaterTower,
            "51001_1002" => Self::ChurchTower,
            "51001_1003" => Self::ObservationTower,
            "51001_1004" => Self::ControlTower,
            "51001_1005" => Self::CoolingTower,
            "51001_1006" => Self::Lighthouse,
            "51001_1007" => Self::FireWatchTower,
            "51001_1008" => Self::BroadcastingOrRadioTower,
            "51001_1009" => Self::CityOrGateTower,
            "51001_1010" => Self::MineHeadframe,
            "51001_1011" => Self::DrillingRig,
            "51001_1012" => Self::PalaceOrCastleTower,
            "51001_9998" => Self::NotSpecifiableFromSourceAlkisAtkis,
            "51001_9999" => Self::OtherTower,
            "51002_1215" => Self::BiogasPlant,
            "51002_1220" => Self::WindTurbine,
            "51002_1230" => Self::SolarPanelInstallation,
            "51002_1250" => Self::Mast,
            "51002_1251" => Self::OverheadPowerLinePylon,
            "51002_1260" => Self::RadioMast,
            "51002_1280" => Self::RadioTelescope,
            "51002_1290" => Self::Chimney,
            "51002_1330" => Self::Crane,
            "51002_1331" => Self::SlewingCrane,
            "51002_1332" => Self::GantryCrane,
            "51002_1333" => Self::OverheadTravellingCrane,
            "51002_1350" => Self::BlastFurnace,
            "51002_1400" => Self::RotaryConverterBuilding,
            "51002_9999" => Self::OtherIndustrialStructure,
            "51003_1201" => Self::Silo,
            "51003_1205" => Self::Tank,
            "51003_1206" => Self::Gasometer,
            "51003_9999" => Self::OtherContainerStructure,
            "51006_1430" => Self::Grandstand,
            "51006_1431" => Self::CoveredGrandstand,
            "51006_1432" => Self::UncoveredGrandstand,
            "51006_1440" => Self::Stadium,
            "51006_1441" => Self::CoveredStadium,
            "51006_1442" => Self::UncoveredStadium,
            "51006_1470" => Self::SkiJumpInrun,
            "51006_1490" => Self::GraduationTower,
            "51006_9999" => Self::OtherSportsOrSpaStructure,
            "51007_1110" => Self::Aqueduct,
            "51007_1210" => Self::WatchTower,
            "51007_1400" => Self::Fortification,
            "51007_1500" => Self::HistoricWall,
            "51007_1510" => Self::CityWall,
            "51007_1520" => Self::OtherHistoricWall,
            "51007_9999" => Self::OtherHistoricStructure,
            "51009_1610" => Self::Canopy,
            "51009_1611" => Self::Carport,
            "51009_1700" => Self::Wall,
            "51009_1750" => Self::Monument,
            "51009_9999" => Self::OtherMiscellaneousStructure,
            "52003_1010" => Self::BoatLift,
            "52003_1020" => Self::ChamberLock,
            "53001_1800" => Self::Bridge,
            "53001_1806" => Self::SwingBridge,
            "53001_1807" => Self::LiftBridge,
            "53001_1808" => Self::Drawbridge,
            "53001_1830" => Self::ElevatedRailwayOrRoad,
            "53001_1890" => Self::LockChamber,
            "53009_2030" => Self::DamWall,
            "53009_2050" => Self::Weir,
            "53009_2060" => Self::SafetyGate,
            "53009_2070" => Self::Sluice,
            "53009_2080" => Self::BarrageOrStormSurgeBarrier,
            "53009_2090" => Self::DrainagePumpingStation,
            other => Self::Other(other.to_string()),
        }
    }

    fn to_code_value(&self) -> &str {
        match self {
            Self::ResidentialBuilding => "31001_1000",
            Self::ResidentialHouse => "31001_1010",
            Self::ResidentialHome => "31001_1020",
            Self::ChildrensHome => "31001_1021",
            Self::SeniorsHome => "31001_1022",
            Self::NursesResidence => "31001_1023",
            Self::StudentOrPupilDormitory => "31001_1024",
            Self::SchoolCampHouse => "31001_1025",
            Self::MixedUseBuildingWithResidential => "31001_1100",
            Self::ResidentialBuildingWithCommunityFacilities => "31001_1110",
            Self::ResidentialBuildingWithRetailAndServices => "31001_1120",
            Self::ResidentialAndAdministrativeBuilding => "31001_1121",
            Self::ResidentialAndOfficeBuilding => "31001_1122",
            Self::ResidentialAndCommercialBuilding => "31001_1123",
            Self::ResidentialBuildingWithTradeAndIndustry => "31001_1130",
            Self::ResidentialAndOperationalBuilding => "31001_1131",
            Self::AgriculturalOrForestryResidentialBuilding => "31001_1210",
            Self::AgriculturalOrForestryResidentialAndOperationalBuilding => "31001_1220",
            Self::FarmHouse => "31001_1221",
            Self::ResidentialAndFarmBuilding => "31001_1222",
            Self::ForestersLodge => "31001_1223",
            Self::RecreationalBuilding => "31001_1310",
            Self::HolidayHouse => "31001_1311",
            Self::WeekendHouse => "31001_1312",
            Self::GardenHouse => "31001_1313",
            Self::BuildingForEconomyOrTrade => "31001_2000",
            Self::BuildingForRetailAndServices => "31001_2010",
            Self::OfficeBuilding => "31001_2020",
            Self::CreditInstitution => "31001_2030",
            Self::InsuranceCompany => "31001_2040",
            Self::CommercialBuilding => "31001_2050",
            Self::DepartmentStore => "31001_2051",
            Self::ShoppingCentre => "31001_2052",
            Self::MarketHall => "31001_2053",
            Self::Shop => "31001_2054",
            Self::Kiosk => "31001_2055",
            Self::Pharmacy => "31001_2056",
            Self::ExhibitionHall => "31001_2060",
            Self::BuildingForAccommodation => "31001_2070",
            Self::HotelMotelOrGuesthouse => "31001_2071",
            Self::YouthHostel => "31001_2072",
            Self::CabinWithOvernightAccommodation => "31001_2073",
            Self::CampsiteBuilding => "31001_2074",
            Self::BuildingForCatering => "31001_2080",
            Self::RestaurantOrInn => "31001_2081",
            Self::CabinWithoutOvernightAccommodation => "31001_2082",
            Self::Canteen => "31001_2083",
            Self::RecreationAndEntertainmentVenue => "31001_2090",
            Self::BanquetHall => "31001_2091",
            Self::Cinema => "31001_2092",
            Self::BowlingAlley => "31001_2093",
            Self::Casino => "31001_2094",
            Self::AmusementArcade => "31001_2095",
            Self::BuildingForTradeAndIndustry => "31001_2100",
            Self::ProductionBuilding => "31001_2110",
            Self::Factory => "31001_2111",
            Self::OperationalBuilding => "31001_2112",
            Self::Brewery => "31001_2113",
            Self::Distillery => "31001_2114",
            Self::Workshop => "31001_2120",
            Self::Sawmill => "31001_2121",
            Self::PetrolStation => "31001_2130",
            Self::CarWashFacility => "31001_2131",
            Self::BuildingForStorage => "31001_2140",
            Self::ColdStore => "31001_2141",
            Self::WarehouseBuilding => "31001_2142",
            Self::StorageHallOrShed => "31001_2143",
            Self::FreightForwardingBuilding => "31001_2150",
            Self::BuildingForResearchPurposes => "31001_2160",
            Self::BuildingForRawMaterialExtraction => "31001_2170",
            Self::Mine => "31001_2171",
            Self::SaltWorks => "31001_2172",
            Self::BuildingForCompanyWelfareFacility => "31001_2180",
            Self::OtherBuildingForTradeAndIndustry => "31001_2200",
            Self::Mill => "31001_2210",
            Self::Windmill => "31001_2211",
            Self::WaterMill => "31001_2212",
            Self::WaterScoopMill => "31001_2213",
            Self::WeatherStation => "31001_2220",
            Self::BuildingForRetailAndServicesWithResidential => "31001_2310",
            Self::BuildingForTradeAndIndustryWithResidential => "31001_2320",
            Self::OperationalBuildingForTransportFacilitiesGeneral => "31001_2400",
            Self::OperationalBuildingForRoadTraffic => "31001_2410",
            Self::RoadMaintenanceDepot => "31001_2411",
            Self::WaitingHall => "31001_2412",
            Self::OperationalBuildingForRailTraffic => "31001_2420",
            Self::RailwayWatchmansHouse => "31001_2421",
            Self::EngineShedOrCarriageHall => "31001_2422",
            Self::SignalBoxOrBlockPost => "31001_2423",
            Self::FreightStationOperationalBuilding => "31001_2424",
            Self::OperationalBuildingForAirTraffic => "31001_2430",
            Self::AircraftHangar => "31001_2431",
            Self::OperationalBuildingForShipTraffic => "31001_2440",
            Self::ShipyardHall => "31001_2441",
            Self::DockHall => "31001_2442",
            Self::LockOperationalBuilding => "31001_2443",
            Self::Boathouse => "31001_2444",
            Self::CablewayOperationalBuilding => "31001_2450",
            Self::CablewayTensioningStation => "31001_2451",
            Self::BuildingForParking => "31001_2460",
            Self::MultiStoreyCarPark => "31001_2461",
            Self::ParkingDeck => "31001_2462",
            Self::Garage => "31001_2463",
            Self::VehicleHall => "31001_2464",
            Self::BuildingForUtilitySupply => "31001_2500",
            Self::BuildingForEnergySupply => "31001_2501",
            Self::BuildingForWaterSupply => "31001_2510",
            Self::Waterworks => "31001_2511",
            Self::PumpStation => "31001_2512",
            Self::WaterTank => "31001_2513",
            Self::BuildingForElectricitySupply => "31001_2520",
            Self::PowerPlant => "31001_2521",
            Self::TransformerStation => "31001_2522",
            Self::ElectricalConverterBuilding => "31001_2523",
            Self::ReactorBuilding => "31001_2527",
            Self::TurbineHouse => "31001_2528",
            Self::BoilerHouse => "31001_2529",
            Self::BuildingForTelecommunications => "31001_2540",
            Self::BuildingAtUndergroundPipelines => "31001_2560",
            Self::BuildingForGasSupply => "31001_2570",
            Self::GasWorks => "31001_2571",
            Self::HeatingPlant => "31001_2580",
            Self::BuildingForSupplyFacility => "31001_2590",
            Self::PumpingPlantNotForWaterSupply => "31001_2591",
            Self::BuildingForWasteDisposal => "31001_2600",
            Self::BuildingForWastewaterDisposal => "31001_2610",
            Self::SewageTreatmentPlantBuilding => "31001_2611",
            Self::Toilet => "31001_2612",
            Self::BuildingForWasteTreatment => "31001_2620",
            Self::RefuseBunker => "31001_2621",
            Self::WasteIncinerationBuilding => "31001_2622",
            Self::LandfillBuilding => "31001_2623",
            Self::BuildingForAgricultureAndForestry => "31001_2700",
            Self::AgriculturalOrForestryOperationalBuilding => "31001_2720",
            Self::Barn => "31001_2721",
            Self::Shed => "31001_2723",
            Self::Stable => "31001_2724",
            Self::BarnAndStable => "31001_2726",
            Self::IntensiveLivestockStable => "31001_2727",
            Self::RidingHall => "31001_2728",
            Self::FarmOutbuilding => "31001_2729",
            Self::AlpineHut => "31001_2732",
            Self::HuntingLodge => "31001_2735",
            Self::Greenhouse => "31001_2740",
            Self::Hothouse => "31001_2741",
            Self::MovableGreenhouse => "31001_2742",
            Self::BuildingForPublicPurposes => "31001_3000",
            Self::AdministrativeBuilding => "31001_3010",
            Self::Parliament => "31001_3011",
            Self::CityHall => "31001_3012",
            Self::PostOffice => "31001_3013",
            Self::CustomsOffice => "31001_3014",
            Self::CourtOfLaw => "31001_3015",
            Self::EmbassyOrConsulate => "31001_3016",
            Self::DistrictAdministration => "31001_3017",
            Self::RegionalGovernment => "31001_3018",
            Self::TaxOffice => "31001_3019",
            Self::BuildingForEducationAndResearch => "31001_3020",
            Self::GeneralEducationSchool => "31001_3021",
            Self::VocationalSchool => "31001_3022",
            Self::UniversityBuilding => "31001_3023",
            Self::ResearchInstitute => "31001_3024",
            Self::BuildingForCulturalPurposes => "31001_3030",
            Self::Palace => "31001_3031",
            Self::TheatreOrOpera => "31001_3032",
            Self::ConcertHall => "31001_3033",
            Self::Museum => "31001_3034",
            Self::BroadcastingBuilding => "31001_3035",
            Self::EventBuilding => "31001_3036",
            Self::Library => "31001_3037",
            Self::CastleOrFortress => "31001_3038",
            Self::BuildingForReligiousPurposes => "31001_3040",
            Self::Church => "31001_3041",
            Self::Synagogue => "31001_3042",
            Self::Chapel => "31001_3043",
            Self::ParishHall => "31001_3044",
            Self::HouseOfWorship => "31001_3045",
            Self::Mosque => "31001_3046",
            Self::Temple => "31001_3047",
            Self::Monastery => "31001_3048",
            Self::BuildingForHealthCare => "31001_3050",
            Self::Hospital => "31001_3051",
            Self::NursingOrCareFacility => "31001_3052",
            Self::MedicalCentreOrPolyclinic => "31001_3053",
            Self::AmbulanceStation => "31001_3054",
            Self::BuildingForSocialPurposes => "31001_3060",
            Self::YouthCentre => "31001_3061",
            Self::CommunityOrClubHouse => "31001_3062",
            Self::SeniorsRecreationCentre => "31001_3063",
            Self::HomelessShelter => "31001_3064",
            Self::NurseryOrKindergarten => "31001_3065",
            Self::AsylumSeekersHome => "31001_3066",
            Self::BuildingForPublicSafetyAndOrder => "31001_3070",
            Self::PoliceStation => "31001_3071",
            Self::FireStation => "31001_3072",
            Self::Barracks => "31001_3073",
            Self::Bunker => "31001_3074",
            Self::Prison => "31001_3075",
            Self::CemeteryBuilding => "31001_3080",
            Self::FuneralHall => "31001_3081",
            Self::Crematorium => "31001_3082",
            Self::ReceptionBuilding => "31001_3090",
            Self::RailwayStationBuilding => "31001_3091",
            Self::AirportBuilding => "31001_3092",
            Self::UndergroundStationBuilding => "31001_3094",
            Self::SuburbanRailStationBuilding => "31001_3095",
            Self::BusStationBuilding => "31001_3097",
            Self::ShippingTerminalBuilding => "31001_3098",
            Self::BuildingForPublicPurposesWithResidential => "31001_3100",
            Self::BuildingForRecreationalPurposes => "31001_3200",
            Self::BuildingForSportsPurposes => "31001_3210",
            Self::SportsOrGymHall => "31001_3211",
            Self::SportsGroundBuilding => "31001_3212",
            Self::BathingBuilding => "31001_3220",
            Self::IndoorSwimmingPool => "31001_3221",
            Self::OutdoorPoolBuilding => "31001_3222",
            Self::StadiumBuilding => "31001_3230",
            Self::BuildingForSpaOperations => "31001_3240",
            Self::MedicalBathingBuilding => "31001_3241",
            Self::Sanatorium => "31001_3242",
            Self::ZooBuilding => "31001_3260",
            Self::ZooReceptionBuilding => "31001_3261",
            Self::AquariumTerrariumOrAviary => "31001_3262",
            Self::AnimalShowHouse => "31001_3263",
            Self::ZooStable => "31001_3264",
            Self::BotanicalGardenBuilding => "31001_3270",
            Self::BotanicalGardenReceptionBuilding => "31001_3271",
            Self::BotanicalGreenhouse => "31001_3272",
            Self::PlantShowHouse => "31001_3273",
            Self::BuildingForOtherRecreationalFacility => "31001_3280",
            Self::ShelterHut => "31001_3281",
            Self::TouristInformationCentre => "31001_3290",
            Self::NotSpecifiableFromSourceAlkis => "31001_9998",
            Self::WaterTower => "51001_1001",
            Self::ChurchTower => "51001_1002",
            Self::ObservationTower => "51001_1003",
            Self::ControlTower => "51001_1004",
            Self::CoolingTower => "51001_1005",
            Self::Lighthouse => "51001_1006",
            Self::FireWatchTower => "51001_1007",
            Self::BroadcastingOrRadioTower => "51001_1008",
            Self::CityOrGateTower => "51001_1009",
            Self::MineHeadframe => "51001_1010",
            Self::DrillingRig => "51001_1011",
            Self::PalaceOrCastleTower => "51001_1012",
            Self::NotSpecifiableFromSourceAlkisAtkis => "51001_9998",
            Self::OtherTower => "51001_9999",
            Self::BiogasPlant => "51002_1215",
            Self::WindTurbine => "51002_1220",
            Self::SolarPanelInstallation => "51002_1230",
            Self::Mast => "51002_1250",
            Self::OverheadPowerLinePylon => "51002_1251",
            Self::RadioMast => "51002_1260",
            Self::RadioTelescope => "51002_1280",
            Self::Chimney => "51002_1290",
            Self::Crane => "51002_1330",
            Self::SlewingCrane => "51002_1331",
            Self::GantryCrane => "51002_1332",
            Self::OverheadTravellingCrane => "51002_1333",
            Self::BlastFurnace => "51002_1350",
            Self::RotaryConverterBuilding => "51002_1400",
            Self::OtherIndustrialStructure => "51002_9999",
            Self::Silo => "51003_1201",
            Self::Tank => "51003_1205",
            Self::Gasometer => "51003_1206",
            Self::OtherContainerStructure => "51003_9999",
            Self::Grandstand => "51006_1430",
            Self::CoveredGrandstand => "51006_1431",
            Self::UncoveredGrandstand => "51006_1432",
            Self::Stadium => "51006_1440",
            Self::CoveredStadium => "51006_1441",
            Self::UncoveredStadium => "51006_1442",
            Self::SkiJumpInrun => "51006_1470",
            Self::GraduationTower => "51006_1490",
            Self::OtherSportsOrSpaStructure => "51006_9999",
            Self::Aqueduct => "51007_1110",
            Self::WatchTower => "51007_1210",
            Self::Fortification => "51007_1400",
            Self::HistoricWall => "51007_1500",
            Self::CityWall => "51007_1510",
            Self::OtherHistoricWall => "51007_1520",
            Self::OtherHistoricStructure => "51007_9999",
            Self::Canopy => "51009_1610",
            Self::Carport => "51009_1611",
            Self::Wall => "51009_1700",
            Self::Monument => "51009_1750",
            Self::OtherMiscellaneousStructure => "51009_9999",
            Self::BoatLift => "52003_1010",
            Self::ChamberLock => "52003_1020",
            Self::Bridge => "53001_1800",
            Self::SwingBridge => "53001_1806",
            Self::LiftBridge => "53001_1807",
            Self::Drawbridge => "53001_1808",
            Self::ElevatedRailwayOrRoad => "53001_1830",
            Self::LockChamber => "53001_1890",
            Self::DamWall => "53009_2030",
            Self::Weir => "53009_2050",
            Self::SafetyGate => "53009_2060",
            Self::Sluice => "53009_2070",
            Self::BarrageOrStormSurgeBarrier => "53009_2080",
            Self::DrainagePumpingStation => "53009_2090",
            Self::Other(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecitygml_core::model::building::values::BuildingFunctionValue;
    use egml::model::basic_types::Code;

    #[test]
    fn round_trips_known_codes() {
        for (code, value) in [
            (
                "31001_1000",
                AdvBuildingFunctionTypeValue::ResidentialBuilding,
            ),
            ("31001_2130", AdvBuildingFunctionTypeValue::PetrolStation),
            (
                "53009_2090",
                AdvBuildingFunctionTypeValue::DrainagePumpingStation,
            ),
        ] {
            assert_eq!(AdvBuildingFunctionTypeValue::from_code_value(code), value);
            assert_eq!(value.to_code_value(), code);
        }
    }

    #[test]
    fn unrecognized_code_in_this_code_space_falls_back_to_other() {
        let value = AdvBuildingFunctionTypeValue::from_code_value("99999_9999");
        assert_eq!(
            value,
            AdvBuildingFunctionTypeValue::Other("99999_9999".to_string())
        );
        assert_eq!(value.to_code_value(), "99999_9999");
    }

    #[test]
    fn interprets_bare_code_with_no_code_space() {
        let wrapped = BuildingFunctionValue::from(Code::new("31001_2130"));
        assert_eq!(
            wrapped.interpret::<AdvBuildingFunctionTypeValue>(),
            Some(AdvBuildingFunctionTypeValue::PetrolStation)
        );
    }

    #[test]
    fn does_not_interpret_a_different_code_space() {
        let wrapped = BuildingFunctionValue::from(Code::with_code_space(
            "https://example.org/local-codes",
            "31001_1000",
        ));
        assert_eq!(wrapped.interpret::<AdvBuildingFunctionTypeValue>(), None);
    }
}
