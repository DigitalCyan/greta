use std::fmt::Display;

use super::fuel::FuelType;

#[derive(Clone)]
pub struct ShipInfo {
    pub name: String,
    pub consumption: f32, // t/km
}

#[derive(Clone)]
pub enum Ship {
    Sevastopol,
    Novorossiysk,
}

impl Ship {
    pub fn get_ship_info(&self) -> ShipInfo {
        match self {
            Ship::Sevastopol => {
                return ShipInfo {
                    name: "Sevastopol".to_string(),
                    consumption: 1.739,
                }
            }
            Ship::Novorossiysk => {
                return ShipInfo {
                    name: "Novorossiysk".to_string(),
                    consumption: 1.088,
                }
            }
        }
    }

    pub fn get_price_for_distance(&self, kilometers: f32, price_liter: f32) -> f32 {
        let fuel_type = FuelType::Methane;

        let consumption = self.get_ship_info().consumption;
        let tons = kilometers * consumption;

        let liters = fuel_type.ton_to_liter(tons);

        liters * price_liter
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_ship_info().name)
    }
}