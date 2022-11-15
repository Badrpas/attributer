use attributer::prelude::*;


struct DummyMod;
impl Modifier for DummyMod {
    fn apply(&self, target: &mut Stats) {
        if let Some(attr) = target.get_mut("health") {
            attr.inc_by(100.);
        }
    }
}

#[test]
fn modifier_applied() {
    let mut stats = Stats::new();
    assert!(stats.get("health").unwrap().value() == 100.);
    let mod_id = stats.add_modifier(Box::new(DummyMod));
    assert!(stats.get("health").unwrap().value() == 200.);
    stats.update_modifiers();
    assert!(stats.get("health").unwrap().value() == 200.);
    stats.remove_modifier(mod_id);
    assert!(stats.get("health").unwrap().value() == 100.);
    stats.update_modifiers();
    assert!(stats.get("health").unwrap().value() == 100.);
}

