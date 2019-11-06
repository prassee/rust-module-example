mod book;
mod entry;
mod stmt;

use book::AccountBook;
use entry::{Entry, EntryType};
use stmt::{acutils, stmts};

pub fn accounting_demo() {
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

    stmts::quater_stmt();
    acutils::pretty_print();
}
