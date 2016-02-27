mod travis;
mod circle;

use std::env;

#[derive(PartialEq, Debug)]
enum Ci {
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
    err!(env::var("CI")).is_some()
}

fn which_ci() -> Ci {
    match (err!(env::var("TRAVIS")), err!(env::var("CIRCLECI"))) {
        (Some(_), None) => Ci::Travis,
        (None, Some(_)) => Ci::Circle,
        _ => Ci::Unknown,
    }
}

#[test]
fn ci_test() {
    assert!(!ci());
    assert_eq!(which_ci(), Ci::Unknown);
}
