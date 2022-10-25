use std::collections::HashMap;

type ID = u64;

#[derive(Debug)]
struct Attribute {
    pub name: String,
    pub base: f32,
    pub current: f32,
}

impl Attribute {
    pub fn new(name: &str, value: f32) -> Self {
        Attribute {
            name: name.to_string(),
            base: value,
            current: value,
        }
    }
}

struct Stats {
    attributes: Vec<Attribute>,
    modifiers: HashMap<ID, Box<dyn Modifier>>,
    auras: HashMap<ID, Box<dyn Aura>>,
    id_counter: ID,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            attributes: vec![
                Attribute::new("Health", 100.),
                Attribute::new("Mana", 0.),
                Attribute::new("Strength", 2.),
            ],
            modifiers: HashMap::new(),
            auras: HashMap::new(),
            id_counter: 0,
        }
    }

    fn next_id(&mut self) -> ID {
        self.id_counter += 1;
        self.id_counter
    }

    pub fn get(&self, name: &str) -> Option<&Attribute> {
        let low = name.to_lowercase();
        self.attributes.iter().find(|&attr| attr.name.to_lowercase() == low)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Attribute> {
        let low = name.to_lowercase();
        self.attributes.iter_mut().find(|attr| (**attr).name.to_lowercase() == low)
    }

    pub fn add_aura(&mut self, mut aura: Box<dyn Aura>) -> ID {
        let id = self.next_id();
        aura.on_enter(self);
        self.auras.insert(id, aura);
        id
    }

    pub fn remove_aura(&mut self, aura_id: ID) {
        if let Some(mut aura) = self.auras.remove(&aura_id) {
            aura.on_exit(self);
        }
    }

    pub fn update_modifiers(&mut self) {
        for attr in &mut self.attributes {
            attr.current = attr.base;
        }
        let mods = std::mem::take(&mut self.modifiers);
            for (_id, modifier) in mods.iter() {
                modifier.apply(self);
            }
        self.modifiers = mods;
    }

    pub fn add_modifier(&mut self, modifier: Box<dyn Modifier>) -> ID {
        let id = self.next_id();
        self.modifiers.insert(id, modifier);
        self.update_modifiers();
        id
    }

    pub fn remove_modifier(&mut self, modifier_id: ID) {
        self.modifiers.remove(&modifier_id);
        self.update_modifiers();
    }
}

trait Modifier {
    // Will run at each update (modifier list change)
    fn apply(&self, target: &mut Stats);
}

struct DummyMod;
impl Modifier for DummyMod {
    fn apply(&self, target: &mut Stats) {
        if let Some(attr) = target.get_mut("health") {
            attr.current += attr.base * 1.2;
        }
    }
}

trait Aura {
    fn on_enter(&mut self, _target: &mut Stats) {}
    fn on_exit(&mut self, _target: &mut Stats) {}

    fn is_tick_enabled(&mut self) -> bool { false }
    fn on_tick(&mut self, _target: &mut Stats) {}

    fn is_timed(&mut self) -> bool { false }
    fn get_duration(&mut self) -> f32 { 0. }
}

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


fn main() {
    let mut stats = Stats::new();
    println!("{:?} {}", stats.get("health"), stats.modifiers.len());

    let aura_id = stats.add_aura(Box::new(DummyAura::default()));

    println!("{:?} {}", stats.get("health"), stats.modifiers.len());
    stats.remove_aura(aura_id);
    println!("{:?} {}", stats.get("health"), stats.modifiers.len());
}



