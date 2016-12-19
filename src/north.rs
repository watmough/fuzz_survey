use std::fmt;
use SurveyMnemonic;
use EnumValues;

// North reference for survey azimuth (surface direction)
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NorthReference {
    GridNorth,
    MagneticNorth,
    TrueNorth,
    UnknownNorthRef,
}

impl fmt::Display for NorthReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            NorthReference::GridNorth => "Grid North",
            NorthReference::MagneticNorth => "Magnetic North",
            NorthReference::TrueNorth => "True North",
            _ => "",
        };
        write!(f, "{}", display)
    }
}

impl EnumValues<NorthReference> for NorthReference {
    fn values() -> Vec<NorthReference> {
        vec![NorthReference::GridNorth,
             NorthReference::MagneticNorth,
             NorthReference::TrueNorth,
             NorthReference::UnknownNorthRef]
    }
}

impl SurveyMnemonic for NorthReference {
    fn mnemonic(&self) -> String {
        let mut mn = String::new();
        match *self {
            NorthReference::GridNorth => mn.push_str("-grid"),
            NorthReference::MagneticNorth => mn.push_str("-mag"),
            NorthReference::TrueNorth => mn.push_str("-true"),
            _ => mn.push_str("-unk"),
        }
        mn
    }
}
