
#[derive(Debug)]
pub struct Attribute {
    name: String,
    base: f32,
    current: f32,
}

impl Attribute {
    pub fn new(name: &str, value: f32) -> Self {
        Attribute {
            name: name.to_string(),
            base: value,
            current: value,
        }
    }

    pub fn name(&self) -> &String { &self.name }
    pub fn value(&self) -> f32 { self.current }
    pub fn base(&self) -> f32 { self.current }

    pub fn reset(&mut self) { 
        self.current = self.base;
    }

    pub fn set_value(&mut self, new_value: f32) {
        self.current = new_value;
    }
    pub fn inc_by(&mut self, x: f32) -> f32 {
        self.current += x;
        self.current
    }
    pub fn mult_by(&mut self, x: f32) -> f32 {
        self.current *= x;
        self.current
    }
    pub fn div_by(&mut self, x: f32) -> f32 {
        self.current /= x;
        self.current
    }

    pub fn base_set_value(&mut self, new_value: f32) {
        self.base = new_value;
    }
    pub fn base_inc_by(&mut self, x: f32) -> f32 {
        self.base += x;
        self.base
    }
    pub fn base_mult_by(&mut self, x: f32) -> f32 {
        self.base *= x;
        self.base
    }
    pub fn base_div_by(&mut self, x: f32) -> f32 {
        self.base /= x;
        self.base
    }
}
