mod travis;
mod circle;

use std::env;

#[derive(PartialEq, Debug)]
enum CiService {
    Travis,
    Circle,
    Unknown,
}

macro_rules! err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => Some(val),
            Err(e) => match e {
                ::std::env::VarError::NotPresent => None,
                _ => panic!(e),
            }
        }
    }
}

fn ci() -> bool {
    err!(env::var("CI")).is_some() ||
    err!(env::var("CONTINUOUS_INTEGRATION")).is_some()
}

fn which_ci() -> CiService {
    match (err!(env::var("TRAVIS")), err!(env::var("CIRCLECI"))) {
        (Some(_), None) => CiService::Travis,
        (None, Some(_)) => CiService::Circle,
        _ => CiService::Unknown,
    }
}

#[test]
fn ci_test() {
    assert!(!ci());
    assert_eq!(which_ci(), CiService::Unknown);
}
