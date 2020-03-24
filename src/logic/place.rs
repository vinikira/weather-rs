use crate::types::place::Coordinates;
use crate::types::place::Place;

impl Place {
    pub fn pretty(&self) -> String {
        use colored::*;

        format!(
            "{id} - {name} {latt_long}\n",
            id = self.id.green().bold(),
            name = self.name.bold(),
            latt_long = self.coordinates.pretty()
        )
    }
}

impl Coordinates {
    pub fn pretty(&self) -> String {
        format!("({}, {})", self.latt, self.long)
    }
}
