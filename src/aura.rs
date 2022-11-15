
use crate::stats::*;

pub trait Aura {
    fn on_enter(&mut self, _target: &mut Stats) {}
    fn on_exit(&mut self, _target: &mut Stats) {}

    fn is_tick_enabled(&mut self) -> bool { false }
    fn on_tick(&mut self, _target: &mut Stats) {}

    fn is_timed(&mut self) -> bool { false }
    fn get_duration(&mut self) -> f32 { 0. }
}

