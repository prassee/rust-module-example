use crate::fridge;
use crate::publik;

pub fn ripen() {
    publik::publik_fnc();
    let ll = publik::LatLong::apply(23.3, 44.5);
    let x = publik::LatLong::apply(223.8, 122.3);
    println!("{:?}", x);
    ll.print_lat_long();
    println!("{:?}", ll);
    fridge::store_fruits();
}
