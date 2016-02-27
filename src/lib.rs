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

macro_rules! lang_version {
    ($lang:expr, $name:ident) => {
        /// **Travis only**: Returns the $lang version that is used.
        fn $name(&self) -> Option<String> {
            match self.service {
                CiService::Travis => err!(env::var(format!("TRAVIS_{}_VERSION", $lang))),
                _ => None,
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

    /// Returns the locale setting, g.e. `en_US.UTF-8`.
    fn lang() -> Option<String> {
        err!(env::var("LANG"))
    }

    /// Returns the search path.
    fn path() -> Option<String> {
        err!(env::var("PATH"))
    }

    /// Returns the path to the users home directory.
    fn home() -> Option<String> {
        err!(env::var("HOME"))
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

    fn branch(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_BRANCH")),
            CiService::Travis => err!(env::var("TRAVIS_BRANCH")),
            _ => None,
        }
    }

    /// **Circle only**
    /// A permanent link to the current build, for example,
    /// https://circleci.com/gh/circleci/frontend/933
    fn build_url(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_BUILD_URL")),
            _ => None,
        }
    }

    /// Returns the build number.
    /// TODO: convert this to a number.
    fn build_id(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_BUILD_NUM")),
            CiService::Travis => err!(env::var("TRAVIS_BUILD_NUMBER")),
            _ => None,
        }
    }

    /// **Travis only**: The absolute path to the directory where the repository
    /// being built has been copied on the worker.
    /// TODO: Return a filesystem path instead?
    fn build_dir(&self) -> Option<String> {
        match self.service {
            CiService::Travis => err!(env::var("TRAVIS_BUILD_DIR")),
            _ => None,
        }
    }

    /// The sha1 hash of the commit being tested.
    fn commit(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_SHA1")),
            CiService::Travis => err!(env::var("TRAVIS_COMMIT")),
            _ => None,
        }
    }

    /// The number of the pull request this build forms part of.
    /// If this build is not part of a pull request, `None` is returned.
    /// TODO: convert this to a number.
    fn pull_request(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_PR_NUMBER")),
            CiService::Travis => {
                let pr = err!(env::var("TRAVIS_PULL_REQUEST")).unwrap_or("false".to_string());
                if pr == "false" {
                    None
                } else {
                    Some(pr)
                }
            }
            _ => None,
        }
    }

    lang_version!("DART", dart);
    lang_version!("GO", go);
    lang_version!("HAXE", haxe);
    lang_version!("JDK", java);
    lang_version!("JULIA", julia);
    lang_version!("NODE", node);
    lang_version!("OTP", otp);
    lang_version!("PERL", perl);
    lang_version!("PHP", php);
    lang_version!("PYTHON", python);
    lang_version!("R", r);
    lang_version!("RUBY", ruby);
    lang_version!("RUST", rust);
    lang_version!("SCALA", scala);
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
