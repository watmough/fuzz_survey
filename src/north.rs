use std::fmt;

// azimuth North reference
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
