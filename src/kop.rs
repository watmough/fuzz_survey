use std::fmt;

// depth of kick off point, feet is assumed for now
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KickOffPoint {
    KOP(u32),
    FromSurface,
    UnknownKOP,
}

impl fmt::Display for KickOffPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KickOffPoint::KOP(depth) => write!(f, "kick off from {}", depth),
            KickOffPoint::FromSurface => write!(f, "From Surface"),
            _ => write!(f, ""),
        }
    }
}
