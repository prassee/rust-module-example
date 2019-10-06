mod Fruits;
mod Storage;
mod Vegies;
mod publik;

use Fruits::Mango;
use Vegies::Onion;

pub fn mangonion() {
    Mango::ripen();
    Onion::chop();
    publik::publik_fnc();
}
