use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct ExplorerPane {
    pub open: bool,
}
