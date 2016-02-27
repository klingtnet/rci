extern crate rci;

use rci::*;

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
