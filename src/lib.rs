mod book;

use book::accountbook::*;
use book::entry::*;

pub fn accounting_demo() {
    // Mango::ripen();
    // Onion::chop();
    // publik_fnc();

    let sal = Entry::from(
        format!("1/10/2019"),
        format!("sal"),
        EntryType::Income,
        1000.0,
    );
    let rent = Entry::from(
        format!("2/10/2019"),
        format!("rent"),
        EntryType::Expenditure,
        250.0,
    );

    AccountBook::from(vec![sal, rent]).find_balance();
}
