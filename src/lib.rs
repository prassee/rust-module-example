mod fruits;
mod publik;
mod storage;
mod vegies;

use fruits::Mango;
use vegies::Onion;

pub fn mangonion() {
    Mango::ripen();
    Onion::chop();
    publik::publik_fnc();
}
