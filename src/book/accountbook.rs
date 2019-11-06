use crate::book::entry::*;

pub struct AccountBook {
    entries: Vec<Entry>,
}

impl AccountBook {
    pub fn from(entries: Vec<Entry>) -> AccountBook {
        AccountBook { entries }
    }
    pub fn find_balance(&self) -> f32 {
        let mut bal = 0.0;
        for ent in self.entries.iter() {
            let amt = match ent.entry_type {
                EntryType::Income => ent.amount,
                EntryType::Expenditure => -ent.amount,
            };
            bal = amt + bal;
        }
        bal
    }
}
