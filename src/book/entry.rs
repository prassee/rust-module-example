pub struct Entry {
    date: String,
    description: String,
    entry: EntryType,
    amount: f32,
}

pub enum EntryType {
    income,
    expenditure,
}

impl Entry {
    pub fn from(date: String, desc: String, entry: EntryType, amt: f32) -> Entry {
        Entry(date, desc, entry, amt)
    }
}
