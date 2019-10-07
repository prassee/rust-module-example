use crate::fridge;
use crate::publik;

pub fn ripen() {
    publik::publik_fnc();
    publik::LatLong::apply(23.3, 44.5);
    fridge::store_fruits();
}
