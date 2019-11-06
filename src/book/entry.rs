pub struct Entry {
    pub date: String,
    pub description: String,
    pub entry_type: EntryType,
    pub amount: f32,
}

pub enum EntryType {
    Income,
    Expenditure,
}

impl Entry {
    pub fn from(date: String, description: String, entry_type: EntryType, amount: f32) -> Entry {
        Entry {
            date,
            description,
            entry_type,
            amount,
        }
    }
}
