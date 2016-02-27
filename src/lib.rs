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

struct Ci {
    service: CiService,
}
impl Ci {
    fn new() -> Option<Self> {
        let s = Ci::which_ci();
        match s {
            CiService::Travis | CiService::Circle => Some(Ci { service: s }),
            _ => None,
        }
    }

    fn which_ci() -> CiService {
        match (err!(env::var("TRAVIS")), err!(env::var("CIRCLECI"))) {
            (Some(_), None) => CiService::Travis,
            (None, Some(_)) => CiService::Circle,
            _ => CiService::Unknown,
        }
    }

    fn is_travis(&self) -> bool {
        match self.service {
            CiService::Travis => true,
            _ => false,
        }
    }

    fn is_circle(&self) -> bool {
        match self.service {
            CiService::Circle => true,
            _ => false,
        }
    }
}

fn ci() -> bool {
    err!(env::var("CI")).is_some() || err!(env::var("CONTINUOUS_INTEGRATION")).is_some()
}

#[test]
fn ci_test_unknown() {
    let ci = Ci::new();
    if ci.is_some() {
        return;
    }
    assert_eq!(Ci::which_ci(), CiService::Unknown);
}
