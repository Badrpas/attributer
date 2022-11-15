
use crate::stats::*;

pub trait Modifier {
    // Will run at each update (modifier list change)
    fn apply(&self, target: &mut Stats);
}



