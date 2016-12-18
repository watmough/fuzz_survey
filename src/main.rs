
// generate a bunch of deviation surveys by permuting various
// lists of options.

#[macro_use]
extern crate itertools;

use std::fmt;

// survey computation method
#[derive(Copy, Clone, Debug, PartialEq)]
enum ComputationMethod {
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

// azimuth North reference
#[derive(Copy, Clone, Debug, PartialEq)]
enum NorthReference {
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

// depth of kick off point, feet is assumed for now
#[derive(Copy, Clone, Debug, PartialEq)]
enum KickOffPoint {
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

fn write_header() {
    println!("UWI,Name,Method,North Reference,Md,Inclination,Azimuth");
}

fn generate_survey_name(survey_name: &str,
                 m: ComputationMethod,
                 r: NorthReference,
                 k: KickOffPoint)
                 -> String {
    let mut name = String::new();
    name.push_str(survey_name);
    match k {
        KickOffPoint::KOP(_) => name.push_str("-3pt"),
        KickOffPoint::FromSurface => name.push_str("-2pt"),
        _ => name.push_str("-unk"),
    };
    match m {
        ComputationMethod::AverageAngle => name.push_str("-avg"),
        ComputationMethod::BalancedAngle => name.push_str("-bal"),
        ComputationMethod::MinimumCurvature => name.push_str("-min"),
        ComputationMethod::RadiusOfCurvature => name.push_str("-rad"),
        ComputationMethod::Tangential => name.push_str("-tang"),
        _ => name.push_str("-unk"),
    };
    match r {
        NorthReference::GridNorth => name.push_str("-grid"),
        NorthReference::MagneticNorth => name.push_str("-mag"),
        NorthReference::TrueNorth => name.push_str("-true"),
        _ => name.push_str("-unk"),
    }
    name
}

// print deviation survey to stdout
fn generate_survey(uwi: &str, m: ComputationMethod, r: NorthReference, k: KickOffPoint) {
    let survey_name = generate_survey_name("test", m, r, k);
    println!("{},{},{},{},{},{},{}", uwi, survey_name, m, r, 0, 0, 0);
    match k {
        KickOffPoint::KOP(depth) => {
            println!("{},{},{},{},{},{},{}", uwi, survey_name, m, r, depth, 0, 0);
        }
        _ => (),
    }
    println!("{},{},{},{},{},{},{}",
             uwi,
             survey_name,
             m,
             r,
             6000,
             90,
             180);
}

fn main() {
    let uwi = "test well";
    let methods = vec![ComputationMethod::AverageAngle,
                       ComputationMethod::BalancedAngle,
                       ComputationMethod::MinimumCurvature,
                       ComputationMethod::RadiusOfCurvature,
                       ComputationMethod::Tangential,
                       ComputationMethod::UnknownCompMethod];

    let references = vec![NorthReference::GridNorth,
                          NorthReference::MagneticNorth,
                          NorthReference::TrueNorth,
                          NorthReference::UnknownNorthRef];

    let kops = vec![KickOffPoint::FromSurface, KickOffPoint::KOP(1000), KickOffPoint::UnknownKOP];

    // print survey to stdout
    write_header();
    for (m, r, k) in iproduct!(methods, references, kops) {
        generate_survey(&uwi, m, r, k);
    }
}
