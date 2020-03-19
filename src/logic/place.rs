use crate::types::place::{Coordinates, LocationType, Place};

impl Place {
    fn new(name: &str, location: LocationType, coords: Coordinates) -> Self {
        Self {
            name: name.to_string(),
            location_type: location,
            coordinates: coords,
        }
    }
}
