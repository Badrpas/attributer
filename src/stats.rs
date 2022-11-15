use std::collections::HashMap;
use super::aura::*;
use super::modifier::*;
use super::attribute::*;
use super::ID;


pub struct Stats {
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
        self.attributes.iter().find(|&attr| attr.name().to_lowercase() == low)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Attribute> {
        let low = name.to_lowercase();
        self.attributes.iter_mut().find(|attr| (**attr).name().to_lowercase() == low)
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

    pub fn update_auras() {
        unimplemented!()
    }

    pub fn update_modifiers(&mut self) {
        for attr in &mut self.attributes {
            attr.reset();
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



