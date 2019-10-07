mod fridge;
mod fruits;
mod publik;
mod vegies;

use fruits::Mango;
use publik::*;
use vegies::Onion;

pub fn mangonion() {
    Mango::ripen();
    Onion::chop();
    publik_fnc();
}
