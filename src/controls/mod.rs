pub mod full;
pub mod mini;
pub mod models;

pub use models::*;

pub trait Keyboard {
    fn process_input(&self) -> Option<Command>;
}
