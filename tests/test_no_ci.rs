extern crate rci;

use rci::*;

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
