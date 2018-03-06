use std::fmt;

/// Property categorizes the unit by use. Not much mention of it in the UCUM
/// HTML spec, but is used throughout the
/// [XML description](http://unitsofmeasure.org/ucum-essence.xml).
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Property {
    Acceleration,
    Acidity,
    Action,
    ActionArea,
    AmountOfAllergen,
    AmountOfFibrinogen,
    AmountOfInfectuousAgent,
    AmountOfInformation,
    AmountOfProliferatingOrganism,
    AmountOfSubstance,
    AmountOfSubstanceDissolvedParticles,
    Arbitrary,
    ArbitraryBiologicActivity,
    ArbitraryELISAUnit,
    Area,
    BiologicActivityAmylase,
    BiologicActivityAnticardiolipinIgA,
    BiologicActivityAnticardiolipinIgG,
    BiologicActivityAnticardiolipinIgM,
    BiologicActivityAntistreptolysinO,
    BiologicActivityFactorVIIIInhibitor,
    BiologicActivityFactorXaInhibitor,
    BiologicActivityPhosphatase,
    BiologicActivityTuberculin,
    BiologicInfectivity,
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
    LinearMassDensity,
    LineicNumber,
    LuminousFlux,
    LuminousIntensity,
    LuminousIntensityDensity,
    MagneticFieldIntensity,
    MagneticFlux,
    MagneticFluxDensity,
    MagneticPermeability,
    MagneticTension,
    Mass,
    MassConcentration,
    MassFraction,
    MetabolicCost,
    Number,
    PlaneAngle,
    Power,
    PowerLevel,
    Pressure,
    PressureLevel,
    ProcedureDefinedAntigen,
    ProcedureDefinedPoliomyelitis,
    ProcedureDefinedProtein,
    ProcedureDefinedRagweedAllergen,
    ProcedureDefinedReferenceAllergen,
    Radioactivity,
    RefractionOfLens,
    RefractionOfPrism,
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

impl Default for Property {
    fn default() -> Property {
        Property::Unclassified
    }
}

impl fmt::Display for Property {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let string = match *self {
            Property::Acceleration => "acceleration",
            Property::Acidity => "acidity",
            Property::Action => "action",
            Property::ActionArea => "action area",
            Property::AmountOfAllergen => {
                "amount of an allergen callibrated through in-vivo testing based \
                 on the ID50EAL method of (intradermal dilution for 50mm sum \
                 of erythema diameters"
            }
            Property::AmountOfFibrinogen => {
                "amount of fibrinogen broken down into the measured d-dimers"
            }
            Property::AmountOfInfectuousAgent => "amount of an infectuous agent",
            Property::AmountOfInformation => "amount of information",
            Property::AmountOfProliferatingOrganism => "amount of a proliferating organism",
            Property::AmountOfSubstance => "amount of substance",
            Property::AmountOfSubstanceDissolvedParticles => {
                "amount of substance (dissolved particles)"
            }
            Property::Arbitrary => "arbitrary",
            Property::ArbitraryBiologicActivity => "arbitrary biologic activity",
            Property::ArbitraryELISAUnit => "arbitrary ELISA activity",
            Property::Area => "area",
            Property::BiologicActivityAmylase => "biologic activity of amylase",
            Property::BiologicActivityAnticardiolipinIgA => {
                "biologic activity of anticardiolipin IgA"
            }
            Property::BiologicActivityAnticardiolipinIgG => {
                "biologic activity of anticardiolipin IgG"
            }
            Property::BiologicActivityAnticardiolipinIgM => {
                "biologic activity of anticardiolipin IgM"
            }
            Property::BiologicActivityAntistreptolysinO => "biologic activity antistreptolysin O",
            Property::BiologicActivityFactorVIIIInhibitor => {
                "biologic activity of factor VIII inhibitor"
            }
            Property::BiologicActivityFactorXaInhibitor => {
                "biologic activity of factor Xa inhibitor (heparin)"
            }
            Property::BiologicActivityPhosphatase => "biologic activity of phosphatase",
            Property::BiologicActivityTuberculin => "biologic activity of tuberculin",
            Property::BiologicInfectivity => {
                "biologic activity (infectivity) of an infectuous agent preparation"
            }
            Property::Brightness => "brightness",
            Property::CatalyticActivity => "catalytic activity",
            Property::DepthOfWater => "depth of water",
            Property::DoseEquivalent => "dose equivalent",
            Property::DryVolume => "dry volume",
            Property::DynamicViscosity => "dynamic viscosity",
            Property::EhrlichUnit => "Ehrlich unit",
            Property::ElectricCapacitance => "electric capacitance",
            Property::ElectricCharge => "electric charge",
            Property::ElectricConductance => "electric conductance",
            Property::ElectricCurrent => "electric current",
            Property::ElectricPermittivity => "electric permittivity",
            Property::ElectricPotential => "electric potential",
            Property::ElectricPotentialLevel => "electric potential level",
            Property::ElectricResistance => "electric resistance",
            Property::Energy => "energy",
            Property::EnergyDose => "energy dose",
            Property::FluidResistance => "fluid resistance",
            Property::FluidVolume => "fluid volume",
            Property::FluxOfMagneticInduction => "flux of magnetic induction",
            Property::Force => "force",
            Property::Fraction => "fraction",
            Property::Frequency => "frequency",
            Property::GaugeOfCatheters => "gauge of catheters",
            Property::HeightOfHorses => "height of horses",
            Property::HomeopathicPotencyHahnemann => "homeopathic potency (Hahnemann)",
            Property::HomeopathicPotencyKorsakov => "homeopathic potency (Korsakov)",
            Property::HomeopathicPotencyRetired => "homeopathic potency (retired)",
            Property::Illuminance => "illuminance",
            Property::Inductance => "inductance",
            Property::IonDose => "ion dose",
            Property::KinematicViscosity => "kinematic viscosity",
            Property::Length => "length",
            Property::Level => "level",
            Property::LinearMassDensity => "linear mass density (of textile thread)",
            Property::LineicNumber => "lineic number",
            Property::LuminousFlux => "luminous flux",
            Property::LuminousIntensity => "luminous intensity",
            Property::LuminousIntensityDensity => "lum. intensity density",
            Property::MagneticFieldIntensity => "magnetic field density",
            Property::MagneticFlux => "magnetic flux",
            Property::MagneticFluxDensity => "magnetic flux density",
            Property::MagneticPermeability => "magnetic permeability",
            Property::MagneticTension => "magnetic tension",
            Property::Mass => "mass",
            Property::MassConcentration => "mass concentration",
            Property::MassFraction => "mass fraction",
            Property::MetabolicCost => "metabolic cost of physical activity",
            Property::Number => "number",
            Property::PlaneAngle => "plane angle",
            Property::Power => "power",
            Property::PowerLevel => "power level",
            Property::Pressure => "pressure",
            Property::PressureLevel => "pressure level",
            Property::ProcedureDefinedAntigen => "procedure defined amount of antigen substance",
            Property::ProcedureDefinedPoliomyelitis => {
                "procedure defined amount of a poliomyelitis d-antigen substance"
            }
            Property::ProcedureDefinedProtein => "procedure defined amount of a protein substance",
            Property::ProcedureDefinedRagweedAllergen => {
                "procedure defined amount of the major allergen of ragweed."
            }
            Property::ProcedureDefinedReferenceAllergen => {
                "procedure defined amount of an allergen using some reference standard"
            }
            Property::Radioactivity => "radioactivity",
            Property::RefractionOfLens => "refraction of a lens",
            Property::RefractionOfPrism => "refraction of a prism",
            Property::SignalTransmissionRate => "signal transmission rate",
            Property::Slope => "slope",
            Property::SolidAngle => "solid angle",
            Property::Temperature => "temperature",
            Property::Time => "time",
            Property::Unclassified => "(unclassified)",
            Property::Velocity => "velocity",
            Property::ViewAreaInMicroscope => "view area in microscope",
            Property::Volume => "volume",
            Property::XRayAttenuation => "x-ray attenuation",
        };

        write!(formatter, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_display() {
        let a = format!("{}", Property::Acceleration);
        assert_eq!(a, "acceleration".to_string());
    }
}
