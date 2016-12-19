use std::fmt;
use EnumValues;
use SurveyMnemonic;

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

impl EnumValues<ComputationMethod> for ComputationMethod {
    fn values() -> Vec<ComputationMethod> {
        vec![ComputationMethod::AverageAngle,
             ComputationMethod::BalancedAngle,
             ComputationMethod::MinimumCurvature,
             ComputationMethod::RadiusOfCurvature,
             ComputationMethod::Tangential,
             ComputationMethod::UnknownCompMethod]
    }
}

impl SurveyMnemonic for ComputationMethod {
    fn mnemonic(&self) -> String {
        let mut mn = String::new();
        match *self {
            ComputationMethod::AverageAngle => mn.push_str("-avg"),
            ComputationMethod::BalancedAngle => mn.push_str("-bal"),
            ComputationMethod::MinimumCurvature => mn.push_str("-min"),
            ComputationMethod::RadiusOfCurvature => mn.push_str("-rad"),
            ComputationMethod::Tangential => mn.push_str("-tan"),
            _ => mn.push_str("-unk"),
        }
        mn
    }
}
