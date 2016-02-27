use std::env;

#[derive(PartialEq, Debug)]
pub enum CiService {
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
        pub fn $name(&self) -> Option<String> {
            match self.service {
                CiService::Travis => err!(env::var(format!("TRAVIS_{}_VERSION", $lang))),
                _ => None,
            }
        }
    }
}

pub struct Ci {
    service: CiService,
}
impl Ci {
    pub fn new() -> Option<Self> {
        let s = Ci::which_ci();
        match s {
            CiService::Travis | CiService::Circle => Some(Ci { service: s }),
            _ => None,
        }
    }

    pub fn which_ci() -> CiService {
        match (err!(env::var("TRAVIS")), err!(env::var("CIRCLECI"))) {
            (Some(_), None) => CiService::Travis,
            (None, Some(_)) => CiService::Circle,
            _ => CiService::Unknown,
        }
    }

    /// Returns the locale setting, g.e. `en_US.UTF-8`.
    pub fn lang() -> Option<String> {
        err!(env::var("LANG"))
    }

    /// Returns the search path.
    pub fn path() -> Option<String> {
        err!(env::var("PATH"))
    }

    /// Returns the path to the users home directory.
    pub fn home() -> Option<String> {
        err!(env::var("HOME"))
    }

    pub fn is_travis(&self) -> bool {
        match self.service {
            CiService::Travis => true,
            _ => false,
        }
    }

    pub fn is_circle(&self) -> bool {
        match self.service {
            CiService::Circle => true,
            _ => false,
        }
    }

    pub fn branch(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_BRANCH")),
            CiService::Travis => err!(env::var("TRAVIS_BRANCH")),
            _ => None,
        }
    }

    /// **Circle only**
    /// A permanent link to the current build, for example,
    /// https://circleci.com/gh/circleci/frontend/933
    pub fn build_url(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_BUILD_URL")),
            _ => None,
        }
    }

    /// Returns the build number.
    /// TODO: convert this to a number.
    pub fn build_id(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_BUILD_NUM")),
            CiService::Travis => err!(env::var("TRAVIS_BUILD_NUMBER")),
            _ => None,
        }
    }

    /// **Travis only**: The absolute path to the directory where the repository
    /// being built has been copied on the worker.
    /// TODO: Return a filesystem path instead?
    pub fn build_dir(&self) -> Option<String> {
        match self.service {
            CiService::Travis => err!(env::var("TRAVIS_BUILD_DIR")),
            _ => None,
        }
    }

    /// The sha1 hash of the commit being tested.
    pub fn commit(&self) -> Option<String> {
        match self.service {
            CiService::Circle => err!(env::var("CIRCLE_SHA1")),
            CiService::Travis => err!(env::var("TRAVIS_COMMIT")),
            _ => None,
        }
    }

    /// The number of the pull request this build forms part of.
    /// If this build is not part of a pull request, `None` is returned.
    /// TODO: convert this to a number.
    pub fn pull_request(&self) -> Option<String> {
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

#[test]
fn test_unknown() {
    assert!(Ci::lang().is_some());
    assert!(Ci::path().is_some());
    assert!(Ci::home().is_some());
    let ci = Ci::new();
    if ci.is_some() {
        return;
    }
    assert_eq!(Ci::which_ci(), CiService::Unknown);
}

#[test]
fn test_travis() {
    assert!(Ci::lang().is_some());
    assert!(Ci::path().is_some());
    assert!(Ci::home().is_some());
    let ci = match Ci::new() {
        None => return,
        Some(ci) => {
            if ci.is_travis() {
                ci
            } else {
                return;
            }
        }
    };
    assert!(ci.is_travis());
    assert!(ci.rust().is_some());
    assert!(ci.commit().is_some());
    assert!(ci.build_dir().is_some());
}

#[test]
fn test_circle() {
    assert!(Ci::lang().is_some());
    assert!(Ci::path().is_some());
    assert!(Ci::home().is_some());
    let ci = match Ci::new() {
        None => return,
        Some(ci) => {
            if ci.is_circle() {
                ci
            } else {
                return;
            }
        }
    };
    assert!(ci.is_circle());
    assert!(ci.commit().is_some());
    assert!(ci.build_url().is_some());
}
