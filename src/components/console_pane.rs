use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct ConsolePane {
    pub open: bool,
}
