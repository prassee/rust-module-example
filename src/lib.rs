mod book;
mod fridge;
mod fruits;
mod publik;
mod vegies;

use book::accountbook::*;
use book::entry::*;
use fruits::Mango;
use publik::*;
use vegies::Onion;

pub fn mangonion() {
    Mango::ripen();
    Onion::chop();
    publik_fnc();

    let sal = Entry::from("10/10/2019", "sal", EntryType::income, 1000.0);
    let rent = Entry::from("10/10/2019", "rent", EntryType::expenditure, 250.0);

    AccountBook::from(vec![sal, rent]).findBalance()
}
