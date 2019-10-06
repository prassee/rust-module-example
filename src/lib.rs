mod Fruits;
mod Storage;
mod Vegies;

use Fruits::Mango;
use Vegies::Onion;

pub fn mangonion() {
    Mango::ripen();
    Onion::chop();
}
