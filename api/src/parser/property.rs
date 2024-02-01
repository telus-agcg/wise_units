//-----------------------------------------------------------------------------
// DO NOT EDIT THIS FILE!
// This is generated by wise_units-atom_generator.
//-----------------------------------------------------------------------------
use crate::parser::Atom;
use std::fmt;
/// Property categorizes the unit by use. Not much mention of it in the UCUM
/// HTML spec, but is used throughout the
/// [XML description](http://unitsofmeasure.org/ucum-essence.xml).
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Property {
    Acceleration,
    Acidity,
    Action,
    ActionArea,
    AmountOfAProliferatingOrganism,
    AmountOfAnAllergenCallibratedThroughInVivoTestingBasedOnTheId50ealMethodOfIntradermalDilutionFor50mmSumOfErythemaDiameters,
    AmountOfAnAllergenCallibratedThroughInVivoTestingUsingTheStallergenesMethod,
    AmountOfAnInfectiousAgent,
    AmountOfFibrinogenBrokenDownIntoTheMeasuredDDimers,
    AmountOfInformation,
    AmountOfSubstance,
    AmountOfSubstanceDissolvedParticles,
    AmplitudeSpectralDensity,
    Arbitrary,
    ArbitraryBiologicActivity,
    ArbitraryElisaUnit,
    Area,
    BiologicActivityAntistreptolysinO,
    BiologicActivityInfectivityOfAnInfectiousAgentPreparation,
    BiologicActivityOfAmylase,
    BiologicActivityOfAnticardiolipinIgA,
    BiologicActivityOfAnticardiolipinIgG,
    BiologicActivityOfAnticardiolipinIgM,
    BiologicActivityOfFactorViiiInhibitor,
    BiologicActivityOfFactorXaInhibitorHeparin,
    BiologicActivityOfPhosphatase,
    BiologicActivityOfTuberculin,
    Brightness,
    CatalyticActivity,
    DepthOfWater,
    DoseEquivalent,
    DryVolume,
    DynamicViscosity,
    EhrlichUnit,
    ElectricCapacitance,
    ElectricCharge,
    ElectricConductance,
    ElectricCurrent,
    ElectricPermittivity,
    ElectricPotential,
    ElectricPotentialLevel,
    ElectricResistance,
    Energy,
    EnergyDose,
    FluidResistance,
    FluidVolume,
    FluxOfMagneticInduction,
    Force,
    Fraction,
    Frequency,
    GaugeOfCatheters,
    HeightOfHorses,
    HomeopathicPotencyHahnemann,
    HomeopathicPotencyKorsakov,
    HomeopathicPotencyRetired,
    Illuminance,
    Inductance,
    IonDose,
    KinematicViscosity,
    Length,
    Level,
    LinearMassDensityOfTextileThread,
    LineicNumber,
    LumIntensityDensity,
    LuminousFlux,
    LuminousIntensity,
    MagneticFieldIntensity,
    MagneticFlux,
    MagneticFluxDensity,
    MagneticPermeability,
    MagneticTension,
    Mass,
    MassConcentration,
    MassFraction,
    MetabolicCostOfPhysicalActivity,
    Number,
    PlaneAngle,
    Power,
    PowerLevel,
    Pressure,
    PressureLevel,
    ProcedureDefinedAmountOfAPoliomyelitisDAntigenSubstance,
    ProcedureDefinedAmountOfAProteinSubstance,
    ProcedureDefinedAmountOfAnAllergenUsingSomeReferenceStandard,
    ProcedureDefinedAmountOfAnAntigenSubstance,
    ProcedureDefinedAmountOfTheMajorAllergenOfRagweed,
    Radioactivity,
    RefractionOfALens,
    RefractionOfAPrism,
    SedimentationCoefficient,
    SignalTransmissionRate,
    Slope,
    SolidAngle,
    Temperature,
    Time,
    Unclassified,
    Velocity,
    ViewAreaInMicroscope,
    Volume,
    XRayAttenuation,
}
impl Property {
    /// All `Atom`s that match the `Property` variant.
    ///
    /// ```
    /// extern crate wise_units;
    /// use wise_units::{Atom, Property};
    ///
    /// assert_eq!(Property::Acidity.atoms(), vec![Atom::PH]);
    /// ```
    ///
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn atoms(self) -> Vec<Atom> {
        match self {
            Self::Acceleration => vec![Atom::Gal, Atom::StandardAccelerationOfFreeFall,],
            Self::Acidity => vec![Atom::PH,],
            Self::Action => vec![Atom::PlanckConstant,],
            Self::ActionArea => vec![Atom::Barn,],
            Self::AmountOfAProliferatingOrganism => vec![Atom::ColonyFormingUnits,],
            Self::AmountOfAnAllergenCallibratedThroughInVivoTestingBasedOnTheId50ealMethodOfIntradermalDilutionFor50mmSumOfErythemaDiameters => {
                vec![Atom::BioequivalentAllergenUnit,]
            }
            Self::AmountOfAnAllergenCallibratedThroughInVivoTestingUsingTheStallergenesMethod => {
                vec![Atom::IndexOfReactivity,]
            }
            Self::AmountOfAnInfectiousAgent => {
                vec![Atom::FocusFormingUnits, Atom::PlaqueFormingUnits,]
            }
            Self::AmountOfFibrinogenBrokenDownIntoTheMeasuredDDimers => {
                vec![Atom::FibrinogenEquivalentUnit,]
            }
            Self::AmountOfInformation => {
                vec![Atom::Bit, Atom::BitLogarithmusDualis, Atom::Byte,]
            }
            Self::AmountOfSubstance => vec![Atom::Equivalents, Atom::Mole,],
            Self::AmountOfSubstanceDissolvedParticles => vec![Atom::Osmole,],
            Self::AmplitudeSpectralDensity => {
                vec![Atom::MeterPerSquareSecondsPerSquareRootOfHertz,]
            }
            Self::Arbitrary => {
                vec![
                    Atom::ArbitraryUnit, Atom::InternationalUnit,
                    Atom::InternationalUnitSecondary, Atom::UnitedStatesPharmacopeiaUnit,
                ]
            }
            Self::ArbitraryBiologicActivity => {
                vec![Atom::KunkelUnit, Atom::MacLaganUnit,]
            }
            Self::ArbitraryElisaUnit => vec![Atom::ElisaUnit,],
            Self::Area => {
                vec![
                    Atom::AcreBritish, Atom::AcreUS, Atom::Are,
                    Atom::CircularMilInternational, Atom::Section,
                    Atom::SquareFootInternational, Atom::SquareInchInternational,
                    Atom::SquareMileUS, Atom::SquareRodUS, Atom::SquareYardInternational,
                    Atom::Township,
                ]
            }
            Self::BiologicActivityAntistreptolysinO => vec![Atom::ToddUnit,],
            Self::BiologicActivityInfectivityOfAnInfectiousAgentPreparation => {
                vec![
                    Atom::CellCultureInfectiousDose, Atom::EmbryoInfectiousDose,
                    Atom::TissueCultureInfectiousDose,
                ]
            }
            Self::BiologicActivityOfAmylase => vec![Atom::DyeUnit, Atom::SomogyiUnit,],
            Self::BiologicActivityOfAnticardiolipinIgA => vec![Atom::AplUnit,],
            Self::BiologicActivityOfAnticardiolipinIgG => vec![Atom::GplUnit,],
            Self::BiologicActivityOfAnticardiolipinIgM => vec![Atom::MplUnit,],
            Self::BiologicActivityOfFactorViiiInhibitor => vec![Atom::BethesdaUnit,],
            Self::BiologicActivityOfFactorXaInhibitorHeparin => {
                vec![Atom::AntiFactorXaUnit,]
            }
            Self::BiologicActivityOfPhosphatase => {
                vec![Atom::BodanskyUnit, Atom::KingArmstrongUnit,]
            }
            Self::BiologicActivityOfTuberculin => vec![Atom::TuberculinUnit,],
            Self::Brightness => vec![Atom::Lambert,],
            Self::CatalyticActivity => vec![Atom::Katal, Atom::Unit,],
            Self::DepthOfWater => vec![Atom::FathomInternational,],
            Self::DoseEquivalent => vec![Atom::RadiationEquivalentMan, Atom::Sievert,],
            Self::DryVolume => {
                vec![
                    Atom::BushelUS, Atom::DryPintUS, Atom::DryQuartUS,
                    Atom::HistoricalWinchesterGallon, Atom::PeckUS,
                ]
            }
            Self::DynamicViscosity => vec![Atom::Poise,],
            Self::EhrlichUnit => vec![Atom::EhrlichUnit,],
            Self::ElectricCapacitance => vec![Atom::Farad,],
            Self::ElectricCharge => vec![Atom::Coulomb, Atom::ElementaryCharge,],
            Self::ElectricConductance => vec![Atom::Mho, Atom::Siemens,],
            Self::ElectricCurrent => vec![Atom::Ampere, Atom::Biot,],
            Self::ElectricPermittivity => vec![Atom::PermittivityOfVacuum,],
            Self::ElectricPotential => vec![Atom::Volt,],
            Self::ElectricPotentialLevel => {
                vec![
                    Atom::Bel10Nanovolt, Atom::BelMicrovolt, Atom::BelMillivolt,
                    Atom::BelVolt,
                ]
            }
            Self::ElectricResistance => vec![Atom::Ohm,],
            Self::Energy => {
                vec![
                    Atom::BritishThermalUnit, Atom::BritishThermalUnitAt39F,
                    Atom::BritishThermalUnitAt59F, Atom::BritishThermalUnitAt60F,
                    Atom::Calorie, Atom::CalorieAt15C, Atom::CalorieAt20C,
                    Atom::Electronvolt, Atom::Erg,
                    Atom::InternationalTableBritishThermalUnit,
                    Atom::InternationalTableCalorie, Atom::Joule,
                    Atom::MeanBritishThermalUnit, Atom::MeanCalorie,
                    Atom::NutritionLabelCalories, Atom::ThermochemicalBritishThermalUnit,
                    Atom::ThermochemicalCalorie,
                ]
            }
            Self::EnergyDose => vec![Atom::Gray, Atom::RadiationAbsorbedDose,],
            Self::FluidResistance => {
                vec![Atom::PeripheralVascularResistanceUnit, Atom::WoodUnit,]
            }
            Self::FluidVolume => {
                vec![
                    Atom::BarrelUS, Atom::CordUS, Atom::FluidDramUS, Atom::FluidOunceUS,
                    Atom::GillUS, Atom::MetricFluidOunce, Atom::MinimUS, Atom::PintUS,
                    Atom::QuartUS, Atom::QueenAnnesWineGallonUS,
                ]
            }
            Self::FluxOfMagneticInduction => vec![Atom::Maxwell,],
            Self::Force => {
                vec![
                    Atom::Dyne, Atom::GramForce, Atom::Newton,
                    Atom::PoundForceAvoirdupois,
                ]
            }
            Self::Fraction => {
                vec![
                    Atom::PartsPerBillion, Atom::PartsPerMillion, Atom::PartsPerThousand,
                    Atom::PartsPerTrillion, Atom::Percent,
                ]
            }
            Self::Frequency => vec![Atom::Hertz,],
            Self::GaugeOfCatheters => vec![Atom::Charriere,],
            Self::HeightOfHorses => vec![Atom::HandInternational,],
            Self::HomeopathicPotencyHahnemann => {
                vec![
                    Atom::HomeopathicPotencyOfCentesimalHahnemannianSeries,
                    Atom::HomeopathicPotencyOfDecimalHahnemannianSeries,
                    Atom::HomeopathicPotencyOfMillesimalHahnemannianSeries,
                    Atom::HomeopathicPotencyOfQuintamillesimalHahnemannianSeries,
                ]
            }
            Self::HomeopathicPotencyKorsakov => {
                vec![
                    Atom::HomeopathicPotencyOfCentesimalKorsakovianSeries,
                    Atom::HomeopathicPotencyOfDecimalKorsakovianSeries,
                    Atom::HomeopathicPotencyOfMillesimalKorsakovianSeries,
                    Atom::HomeopathicPotencyOfQuintamillesimalKorsakovianSeries,
                ]
            }
            Self::HomeopathicPotencyRetired => {
                vec![
                    Atom::HomeopathicPotencyOfCentesimalSeriesRetired,
                    Atom::HomeopathicPotencyOfDecimalSeriesRetired,
                    Atom::HomeopathicPotencyOfMillesimalSeriesRetired,
                    Atom::HomeopathicPotencyOfQuintamillesimalSeriesRetired,
                ]
            }
            Self::Illuminance => vec![Atom::Lux, Atom::Phot,],
            Self::Inductance => vec![Atom::Henry,],
            Self::IonDose => vec![Atom::Roentgen,],
            Self::KinematicViscosity => vec![Atom::Stokes,],
            Self::Length => {
                vec![
                    Atom::Angstrom, Atom::AstronomicUnit, Atom::Cicero, Atom::Didot,
                    Atom::FathomBritish, Atom::FathomUS, Atom::FootBritish,
                    Atom::FootInternational, Atom::FootUS, Atom::FurlongUS,
                    Atom::GuntersChainBritish, Atom::GuntersChainUS, Atom::InchBritish,
                    Atom::InchInternational, Atom::InchUS, Atom::LightYear, Atom::Ligne,
                    Atom::Line, Atom::LinkForGuntersChainBritish,
                    Atom::LinkForGuntersChainUS, Atom::LinkForRamdensChainUS,
                    Atom::Meter, Atom::MilInternational, Atom::MilUS, Atom::MileBritish,
                    Atom::MileInternational, Atom::MileUS, Atom::NauticalMileBritish,
                    Atom::NauticalMileInternational, Atom::PaceBritish, Atom::Parsec,
                    Atom::Pica, Atom::Pied, Atom::Point, Atom::Pouce, Atom::PrintersPica,
                    Atom::PrintersPoint, Atom::RamdensChainUS, Atom::RodBritish,
                    Atom::RodUS, Atom::Smoot, Atom::YardBritish, Atom::YardInternational,
                    Atom::YardUS,
                ]
            }
            Self::Level => vec![Atom::Bel, Atom::Neper,],
            Self::LinearMassDensityOfTextileThread => vec![Atom::Denier, Atom::Tex,],
            Self::LineicNumber => vec![Atom::Kayser, Atom::MeshInternational,],
            Self::LumIntensityDensity => vec![Atom::Stilb,],
            Self::LuminousFlux => vec![Atom::Lumen,],
            Self::LuminousIntensity => vec![Atom::Candela,],
            Self::MagneticFieldIntensity => vec![Atom::Oersted,],
            Self::MagneticFlux => vec![Atom::Weber,],
            Self::MagneticFluxDensity => vec![Atom::Gauss, Atom::Tesla,],
            Self::MagneticPermeability => vec![Atom::PermeabilityOfVacuum,],
            Self::MagneticTension => vec![Atom::Gilbert,],
            Self::Mass => {
                vec![
                    Atom::DramApothecaries, Atom::DramAvoirdupois, Atom::ElectronMass,
                    Atom::Grain, Atom::Gram, Atom::LongHunderdweightAvoirdupois,
                    Atom::LongTonAvoirdupois, Atom::MetricCarat, Atom::MetricOunce,
                    Atom::OunceApothecaries, Atom::OunceAvoirdupois, Atom::OunceTroy,
                    Atom::PennyweightTroy, Atom::PoundApothecaries,
                    Atom::PoundAvoirdupois, Atom::PoundTroy, Atom::ProtonMass,
                    Atom::ScrupleApothecaries, Atom::ShortHundredweightAvoirdupois,
                    Atom::ShortTonAvoirdupois, Atom::StoneAvoirdupois, Atom::Tonne,
                    Atom::UnifiedAtomicMassUnit,
                ]
            }
            Self::MassConcentration => vec![Atom::GramPercent,],
            Self::MassFraction => vec![Atom::CaratOfGoldAlloys,],
            Self::MetabolicCostOfPhysicalActivity => vec![Atom::MetabolicEquivalent,],
            Self::Number => {
                vec![
                    Atom::TheNumberPi, Atom::TheNumberTenForArbitraryPowersCaret,
                    Atom::TheNumberTenForArbitraryPowersStar,
                ]
            }
            Self::PlaneAngle => {
                vec![
                    Atom::Circle, Atom::Degree, Atom::Gon, Atom::MinuteAngle,
                    Atom::Radian, Atom::SecondAngle,
                ]
            }
            Self::Power => vec![Atom::Horsepower, Atom::Watt,],
            Self::PowerLevel => vec![Atom::BelKilowatt, Atom::BelWatt,],
            Self::Pressure => {
                vec![
                    Atom::Bar, Atom::InchOfMercuryColumn, Atom::InchOfWaterColumn,
                    Atom::MeterOfMercuryColumn, Atom::MeterOfWaterColumn, Atom::Pascal,
                    Atom::PoundPerSqareInch, Atom::StandardAtmosphere,
                    Atom::TechnicalAtmosphere,
                ]
            }
            Self::PressureLevel => vec![Atom::BelSoundPressure,],
            Self::ProcedureDefinedAmountOfAPoliomyelitisDAntigenSubstance => {
                vec![Atom::DAntigenUnit,]
            }
            Self::ProcedureDefinedAmountOfAProteinSubstance => {
                vec![Atom::ProteinNitrogenUnit,]
            }
            Self::ProcedureDefinedAmountOfAnAllergenUsingSomeReferenceStandard => {
                vec![Atom::AllergenUnit,]
            }
            Self::ProcedureDefinedAmountOfAnAntigenSubstance => {
                vec![Atom::LimitOfFlocculation,]
            }
            Self::ProcedureDefinedAmountOfTheMajorAllergenOfRagweed => {
                vec![Atom::AllergenUnitForAmbrosiaArtemisiifolia,]
            }
            Self::Radioactivity => vec![Atom::Becquerel, Atom::Curie,],
            Self::RefractionOfALens => vec![Atom::Diopter,],
            Self::RefractionOfAPrism => vec![Atom::PrismDiopter,],
            Self::SedimentationCoefficient => vec![Atom::SvedbergUnit,],
            Self::SignalTransmissionRate => vec![Atom::Baud,],
            Self::Slope => vec![Atom::PercentOfSlope,],
            Self::SolidAngle => vec![Atom::Spere, Atom::Steradian,],
            Self::Temperature => {
                vec![
                    Atom::DegreeCelsius, Atom::DegreeFahrenheit, Atom::DegreeRankine,
                    Atom::DegreeReaumur, Atom::Kelvin,
                ]
            }
            Self::Time => {
                vec![
                    Atom::Day, Atom::Hour, Atom::MeanGregorianMonth,
                    Atom::MeanGregorianYear, Atom::MeanJulianMonth, Atom::MeanJulianYear,
                    Atom::Minute, Atom::Month, Atom::Second, Atom::SynodalMonth,
                    Atom::TropicalYear, Atom::Week, Atom::Year,
                ]
            }
            Self::Unclassified => {
                vec![Atom::BoltzmannConstant, Atom::NewtonianConstantOfGravitation,]
            }
            Self::Velocity => {
                vec![Atom::KnotBritish, Atom::KnotInternational, Atom::VelocityOfLight,]
            }
            Self::ViewAreaInMicroscope => {
                vec![Atom::HighPowerField, Atom::LowPowerField,]
            }
            Self::Volume => {
                vec![
                    Atom::BoardFootInternational, Atom::BushelBritish,
                    Atom::CordInternational, Atom::CubicFootInternational,
                    Atom::CubicInchInternational, Atom::CubicYardInternational,
                    Atom::CupUS, Atom::Drop, Atom::FluidDramBritish,
                    Atom::FluidOunceBritish, Atom::GallonBritish, Atom::GillBritish,
                    Atom::Liter, Atom::LiterSecondary, Atom::MetricCup,
                    Atom::MetricTablespoon, Atom::MetricTeaspoon, Atom::MinimBritish,
                    Atom::PeckBritish, Atom::PintBritish, Atom::QuartBritish,
                    Atom::Stere, Atom::TablespoonUS, Atom::TeaspoonUS,
                ]
            }
            Self::XRayAttenuation => vec![Atom::HounsfieldUnit,],
        }
    }
}
impl Default for Property {
    fn default() -> Self {
        Self::Unclassified
    }
}
impl fmt::Display for Property {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Acceleration => "Acceleration",
            Self::Acidity => "Acidity",
            Self::Action => "Action",
            Self::ActionArea => "ActionArea",
            Self::AmountOfAProliferatingOrganism => "AmountOfAProliferatingOrganism",
            Self::AmountOfAnAllergenCallibratedThroughInVivoTestingBasedOnTheId50ealMethodOfIntradermalDilutionFor50mmSumOfErythemaDiameters => {
                "AmountOfAnAllergenCallibratedThroughInVivoTestingBasedOnTheId50ealMethodOfIntradermalDilutionFor50mmSumOfErythemaDiameters"
            }
            Self::AmountOfAnAllergenCallibratedThroughInVivoTestingUsingTheStallergenesMethod => {
                "AmountOfAnAllergenCallibratedThroughInVivoTestingUsingTheStallergenesMethod"
            }
            Self::AmountOfAnInfectiousAgent => "AmountOfAnInfectiousAgent",
            Self::AmountOfFibrinogenBrokenDownIntoTheMeasuredDDimers => {
                "AmountOfFibrinogenBrokenDownIntoTheMeasuredDDimers"
            }
            Self::AmountOfInformation => "AmountOfInformation",
            Self::AmountOfSubstance => "AmountOfSubstance",
            Self::AmountOfSubstanceDissolvedParticles => {
                "AmountOfSubstanceDissolvedParticles"
            }
            Self::AmplitudeSpectralDensity => "AmplitudeSpectralDensity",
            Self::Arbitrary => "Arbitrary",
            Self::ArbitraryBiologicActivity => "ArbitraryBiologicActivity",
            Self::ArbitraryElisaUnit => "ArbitraryElisaUnit",
            Self::Area => "Area",
            Self::BiologicActivityAntistreptolysinO => {
                "BiologicActivityAntistreptolysinO"
            }
            Self::BiologicActivityInfectivityOfAnInfectiousAgentPreparation => {
                "BiologicActivityInfectivityOfAnInfectiousAgentPreparation"
            }
            Self::BiologicActivityOfAmylase => "BiologicActivityOfAmylase",
            Self::BiologicActivityOfAnticardiolipinIgA => {
                "BiologicActivityOfAnticardiolipinIgA"
            }
            Self::BiologicActivityOfAnticardiolipinIgG => {
                "BiologicActivityOfAnticardiolipinIgG"
            }
            Self::BiologicActivityOfAnticardiolipinIgM => {
                "BiologicActivityOfAnticardiolipinIgM"
            }
            Self::BiologicActivityOfFactorViiiInhibitor => {
                "BiologicActivityOfFactorViiiInhibitor"
            }
            Self::BiologicActivityOfFactorXaInhibitorHeparin => {
                "BiologicActivityOfFactorXaInhibitorHeparin"
            }
            Self::BiologicActivityOfPhosphatase => "BiologicActivityOfPhosphatase",
            Self::BiologicActivityOfTuberculin => "BiologicActivityOfTuberculin",
            Self::Brightness => "Brightness",
            Self::CatalyticActivity => "CatalyticActivity",
            Self::DepthOfWater => "DepthOfWater",
            Self::DoseEquivalent => "DoseEquivalent",
            Self::DryVolume => "DryVolume",
            Self::DynamicViscosity => "DynamicViscosity",
            Self::EhrlichUnit => "EhrlichUnit",
            Self::ElectricCapacitance => "ElectricCapacitance",
            Self::ElectricCharge => "ElectricCharge",
            Self::ElectricConductance => "ElectricConductance",
            Self::ElectricCurrent => "ElectricCurrent",
            Self::ElectricPermittivity => "ElectricPermittivity",
            Self::ElectricPotential => "ElectricPotential",
            Self::ElectricPotentialLevel => "ElectricPotentialLevel",
            Self::ElectricResistance => "ElectricResistance",
            Self::Energy => "Energy",
            Self::EnergyDose => "EnergyDose",
            Self::FluidResistance => "FluidResistance",
            Self::FluidVolume => "FluidVolume",
            Self::FluxOfMagneticInduction => "FluxOfMagneticInduction",
            Self::Force => "Force",
            Self::Fraction => "Fraction",
            Self::Frequency => "Frequency",
            Self::GaugeOfCatheters => "GaugeOfCatheters",
            Self::HeightOfHorses => "HeightOfHorses",
            Self::HomeopathicPotencyHahnemann => "HomeopathicPotencyHahnemann",
            Self::HomeopathicPotencyKorsakov => "HomeopathicPotencyKorsakov",
            Self::HomeopathicPotencyRetired => "HomeopathicPotencyRetired",
            Self::Illuminance => "Illuminance",
            Self::Inductance => "Inductance",
            Self::IonDose => "IonDose",
            Self::KinematicViscosity => "KinematicViscosity",
            Self::Length => "Length",
            Self::Level => "Level",
            Self::LinearMassDensityOfTextileThread => "LinearMassDensityOfTextileThread",
            Self::LineicNumber => "LineicNumber",
            Self::LumIntensityDensity => "LumIntensityDensity",
            Self::LuminousFlux => "LuminousFlux",
            Self::LuminousIntensity => "LuminousIntensity",
            Self::MagneticFieldIntensity => "MagneticFieldIntensity",
            Self::MagneticFlux => "MagneticFlux",
            Self::MagneticFluxDensity => "MagneticFluxDensity",
            Self::MagneticPermeability => "MagneticPermeability",
            Self::MagneticTension => "MagneticTension",
            Self::Mass => "Mass",
            Self::MassConcentration => "MassConcentration",
            Self::MassFraction => "MassFraction",
            Self::MetabolicCostOfPhysicalActivity => "MetabolicCostOfPhysicalActivity",
            Self::Number => "Number",
            Self::PlaneAngle => "PlaneAngle",
            Self::Power => "Power",
            Self::PowerLevel => "PowerLevel",
            Self::Pressure => "Pressure",
            Self::PressureLevel => "PressureLevel",
            Self::ProcedureDefinedAmountOfAPoliomyelitisDAntigenSubstance => {
                "ProcedureDefinedAmountOfAPoliomyelitisDAntigenSubstance"
            }
            Self::ProcedureDefinedAmountOfAProteinSubstance => {
                "ProcedureDefinedAmountOfAProteinSubstance"
            }
            Self::ProcedureDefinedAmountOfAnAllergenUsingSomeReferenceStandard => {
                "ProcedureDefinedAmountOfAnAllergenUsingSomeReferenceStandard"
            }
            Self::ProcedureDefinedAmountOfAnAntigenSubstance => {
                "ProcedureDefinedAmountOfAnAntigenSubstance"
            }
            Self::ProcedureDefinedAmountOfTheMajorAllergenOfRagweed => {
                "ProcedureDefinedAmountOfTheMajorAllergenOfRagweed"
            }
            Self::Radioactivity => "Radioactivity",
            Self::RefractionOfALens => "RefractionOfALens",
            Self::RefractionOfAPrism => "RefractionOfAPrism",
            Self::SedimentationCoefficient => "SedimentationCoefficient",
            Self::SignalTransmissionRate => "SignalTransmissionRate",
            Self::Slope => "Slope",
            Self::SolidAngle => "SolidAngle",
            Self::Temperature => "Temperature",
            Self::Time => "Time",
            Self::Unclassified => "Unclassified",
            Self::Velocity => "Velocity",
            Self::ViewAreaInMicroscope => "ViewAreaInMicroscope",
            Self::Volume => "Volume",
            Self::XRayAttenuation => "XRayAttenuation",
        };
        write!(formatter, "{string}")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_display() {
        let a = format!("{}", Property::Acceleration);
        assert_eq!(&a, "Acceleration");
    }
}
