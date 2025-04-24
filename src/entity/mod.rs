pub mod player;
pub mod zombie;
use std::fmt::Debug;

pub trait Entity: Debug {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn id(&self) -> u32;
    fn entity_type(&self) -> String;
}