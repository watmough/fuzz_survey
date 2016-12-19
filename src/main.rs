
// generate a bunch of deviation surveys by permuting various
// lists of options.

#[macro_use]
extern crate itertools;

mod kop;                // kick-off point
mod method;             // survey computation method
mod north;              // north reference
mod survey;             // utilities to write out header and survey

use kop::KickOffPoint;
use method::ComputationMethod;
use north::NorthReference;
use survey::{write_survey_header, write_survey};

// provide a vec of my enum values
trait EnumValues<T> {
    fn values() -> Vec<T>;
}

// provide a short name from enums
trait SurveyMnemonic {
    fn mnemonic(&self) -> String;
}

fn main() {
    // this should really be specified on the command-line
    let uwi = "test well";

    // get vecs of survey computation methods and north reference methods
    let methods = ComputationMethod::values();
    let references = NorthReference::values();

    // KOP at surface makes a 2pt survey, KOP with depth gives a 3pt survey
    let kops = vec![KickOffPoint::FromSurface, KickOffPoint::KOP(1000), KickOffPoint::UnknownKOP];

    // write a survey header followed by 2pt and 3pt surveys generated
    // with cartesian product of above options
    write_survey_header();
    for (m, r, k) in iproduct!(methods, references, kops) {
        write_survey(&uwi, m, r, k);
    }
}
