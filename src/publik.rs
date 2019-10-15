pub fn publik_fnc() {}

#[derive(Debug)]
pub struct LatLong {
    pub lat: f32,
    pub long: f32,
}

impl LatLong {
    pub fn apply(lat: f32, long: f32) -> LatLong {
        LatLong { lat, long }
    }
    pub fn print_lat_long(&self) {
        println!("{} , {}", self.lat, self.long);
    }
}
