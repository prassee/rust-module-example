use crate::book::entry::*;

pub struct AccountBook {
    entries: Vec<Entry>,
}

impl AccountBook {
    pub fn from(entries: Vec<Entry>) -> AccountBook {
        AccountBook(entries)
    }
    pub fn findBalance(&self) -> f32 {
        let mut bal = 0.0;
        for ent in self.entries {
            let amt = match ent.entry {
                EntryType::income => ent.amount,
                EntryType::expenditure => -ent.amount,
            };
            bal = amt + bal;
        }
        bal
    }
}
