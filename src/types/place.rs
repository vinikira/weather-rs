pub struct Place {
    pub name: String,
    pub location_type: LocationType,
    pub coordinates: Coordinates,
}

pub struct Coordinates {
    pub latt: CoordinateUnit,
    pub long: CoordinateUnit,
}

pub type CoordinateUnit = f32;

pub enum LocationType {
    City,
    RegionOrStateOrProvince,
    Country,
    Continent,
}
