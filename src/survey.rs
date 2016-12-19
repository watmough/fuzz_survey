
use kop::KickOffPoint;
use method::ComputationMethod;
use north::NorthReference;
use SurveyMnemonic;

// given a prefix and options, make a mnemonic survey name
fn generate_survey_name(survey_prefix: &str,
                        method: ComputationMethod,
                        north_ref: NorthReference,
                        kop: KickOffPoint)
                        -> String {
    let mut name = String::new();
    name.push_str(survey_prefix);
    name.push_str(&kop.mnemonic());
    name.push_str(&method.mnemonic());
    name.push_str(&north_ref.mnemonic());
    name
}

// write just the survey header to stdout
pub fn write_survey_header() {
    println!("UWI,Name,Method,North Reference,Md,Inclination,Azimuth");
}

// write a single 2pt or 3pt survey, depending on whether we have a kop
pub fn write_survey(uwi: &str,
                    method: ComputationMethod,
                    north_ref: NorthReference,
                    kop: KickOffPoint) {
    let survey_name = generate_survey_name("test", method, north_ref, kop);
    println!("{},{},{},{},{},{},{}",
             uwi,
             survey_name,
             method,
             north_ref,
             0,
             0,
             0);
    if let KickOffPoint::KOP(depth) = kop {
        println!("{},{},{},{},{},{},{}",
                 uwi,
                 survey_name,
                 method,
                 north_ref,
                 depth,
                 0,
                 0);
    }
    println!("{},{},{},{},{},{},{}",
             uwi,
             survey_name,
             method,
             north_ref,
             6000,
             90,
             180);
}
