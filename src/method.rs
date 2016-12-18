use std::fmt;

// survey computation method
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ComputationMethod {
    AverageAngle,
    BalancedAngle,
    MinimumCurvature,
    RadiusOfCurvature,
    Tangential,
    UnknownCompMethod,
}

impl fmt::Display for ComputationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            ComputationMethod::AverageAngle => "Average Angle",
            ComputationMethod::BalancedAngle => "Balanced Angle",
            ComputationMethod::MinimumCurvature => "Minium Curvature",
            ComputationMethod::RadiusOfCurvature => "Radius of Curvature",
            ComputationMethod::Tangential => "Tangential",
            _ => "",
        };
        write!(f, "{}", display)
    }
}