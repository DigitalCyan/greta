pub enum FuelType {
    Methane,
}

impl FuelType {
    /// Obtain a L to Kg conversion factor
    pub fn get_conversion_factor(&self) -> f32 {
        match self {
            FuelType::Methane => 0.465,
        }
    }

    /// Convert liters of fuel into tons of fuel
    pub fn liter_to_ton(&self, liters: f32) -> f32 {
        let factor = self.get_conversion_factor();

        liters * (factor / 1000.0)
    }

    /// Convert tons of fuel into liters of fuel
    pub fn ton_to_liter(&self, tons: f32) -> f32 {
        let factor = self.get_conversion_factor();

        tons / (factor / 1000.0)
    }

    /// Get somewhat average price of fuel type for early 2023 [Eur/L]
    pub fn get_price(&self) -> f32 {
        match self {
            FuelType::Methane => 1.2,
        }
    }
}
