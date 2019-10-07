pub fn publik_fnc() {}

pub struct LatLong {
    pub lat: f32,
    pub long: f32,
}

impl LatLong {
    pub fn apply(lat: f32, long: f32) -> LatLong {
        LatLong { lat, long }
    }
}
