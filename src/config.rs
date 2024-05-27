use serde::Deserialize;

use crate::controls::Keyboard;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub keyboard: Keyboard,
}
