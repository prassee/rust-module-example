mod entry;
use entry::*;

pub struct AccountBook {
    entries: Vec<Entry>,
}

impl AccountBook {
    pub fn findBalance(&self) -> f32 {
        100.0
    }
}
