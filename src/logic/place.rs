use crate::types::Coordinates;
use crate::types::Place;

impl std::fmt::Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::*;

        writeln!(
            f,
            "{id: >10} {name: <30} {latt_long: <10}",
            id = self.id.green().bold(),
            name = self.name.bold(),
            latt_long = self.coordinates.to_string()
        )
    }
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.latt, self.long)
    }
}
