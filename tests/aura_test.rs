use attributer::prelude::*;

#[derive(Default)]
struct DummyAura {
    mod_id: ID,
}
impl Aura for DummyAura {
    fn on_enter(&mut self, target: &mut Stats) {
        println!("DummyAura on_enter {}", self.mod_id);
        self.mod_id = target.add_modifier(Box::new(DummyMod));
    }

    fn on_exit(&mut self, target: &mut Stats) {
        target.remove_modifier(self.mod_id);
    }
}

struct DummyMod;
impl Modifier for DummyMod {
    fn apply(&self, target: &mut Stats) {
        if let Some(attr) = target.get_mut("health") {
            attr.inc_by(attr.base() * 1.2);
        }
    }
}


#[test]
fn test_aura_with_modifier() {
    let mut stats = Stats::new();
    assert!(stats.get("health").unwrap().value() == 100.);

    let aura_id = stats.add_aura(Box::new(DummyAura::default()));

    assert!(stats.get("health").unwrap().value() == 220.);
    stats.remove_aura(aura_id);
    assert!(stats.get("health").unwrap().value() == 100.);
}

