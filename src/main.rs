
// generate a bunch of deviation surveys by permuting various
// lists of options.

#[macro_use]
extern crate itertools;

mod kop;                // kick-off point
mod method;             // survey computation method
mod north;              // north reference

use kop::KickOffPoint;
use method::ComputationMethod;
use north::NorthReference;

// given a prefix and options, make a mnemonic survey name
fn generate_survey_name(survey_prefix: &str,
                        m: ComputationMethod,
                        r: NorthReference,
                        k: KickOffPoint)
                        -> String {
    let mut name = String::new();
    name.push_str(survey_prefix);
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


fn write_survey_header() {
    println!("UWI,Name,Method,North Reference,Md,Inclination,Azimuth");
}

// write a single 2pt or 3pt survey
fn write_survey(uwi: &str, m: ComputationMethod, r: NorthReference, k: KickOffPoint) {
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
    // this should specified on the command-line
    let uwi = "test well";

    // survey computation methods
    let methods = vec![ComputationMethod::AverageAngle,
                       ComputationMethod::BalancedAngle,
                       ComputationMethod::MinimumCurvature,
                       ComputationMethod::RadiusOfCurvature,
                       ComputationMethod::Tangential,
                       ComputationMethod::UnknownCompMethod];

    // north reference methods
    let references = vec![NorthReference::GridNorth,
                          NorthReference::MagneticNorth,
                          NorthReference::TrueNorth,
                          NorthReference::UnknownNorthRef];

    // KOP at surface makes a 2pt survey, KOP with depth gives a 3pt survey
    let kops = vec![KickOffPoint::FromSurface, KickOffPoint::KOP(1000), KickOffPoint::UnknownKOP];

    // write a survey header followed by 2pt and 3pt surveys generated
    // with cartesian product of above options
    write_survey_header();
    for (m, r, k) in iproduct!(methods, references, kops) {
        write_survey(&uwi, m, r, k);
    }
}
