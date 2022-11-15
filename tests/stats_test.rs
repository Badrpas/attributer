use attributer::prelude::*;


#[test]
fn stat_get_health_ok() {
    let stats = Stats::new();
    assert!(stats.get("health").is_some());
}
#[test]
fn stat_get_health_different_case() {
    let stats = Stats::new();
    assert!(stats.get("hEAltH").is_some());
}
#[test]
fn stat_get_mut_health_ok() {
    let mut stats = Stats::new();
    assert!(stats.get_mut("health").is_some());
}
#[test]
fn stat_get_mut_health_different_case() {
    let mut stats = Stats::new();
    assert!(stats.get_mut("hEAltH").is_some());
}


