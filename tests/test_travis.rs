extern crate rci;

use rci::*;

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
