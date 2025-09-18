// ============================================
// Kata 1: Temperature Converter
// Focus: Good Naming, Single Responsibility, Type Safety
// ============================================

use std::fmt;

/// Strongly-typed representation of temperature units
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl TemperatureUnit {
    /// Convert to Celsius
    pub fn to_celsius(&self) -> f64 {
        match self {
            TemperatureUnit::Celsius(c) => *c,
            TemperatureUnit::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
            TemperatureUnit::Kelvin(k) => k - 273.15,
        }
    }

    /// Convert to Fahrenheit
    pub fn to_fahrenheit(&self) -> f64 {
        match self {
            TemperatureUnit::Celsius(c) => c * 9.0 / 5.0 + 32.0,
            TemperatureUnit::Fahrenheit(f) => *f,
            TemperatureUnit::Kelvin(k) => (k - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }

    /// Convert to Kelvin
    pub fn to_kelvin(&self) -> f64 {
        match self {
            TemperatureUnit::Celsius(c) => c + 273.15,
            TemperatureUnit::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0 + 273.15,
            TemperatureUnit::Kelvin(k) => *k,
        }
    }

    /// Convert to a specified unit
    pub fn convert_to(&self, target: TemperatureScale) -> TemperatureUnit {
        match target {
            TemperatureScale::Celsius => TemperatureUnit::Celsius(self.to_celsius()),
            TemperatureScale::Fahrenheit => TemperatureUnit::Fahrenheit(self.to_fahrenheit()),
            TemperatureScale::Kelvin => TemperatureUnit::Kelvin(self.to_kelvin()),
        }
    }
}

/// Temperature unit type (without value)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureScale {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureUnit::Celsius(temp) => write!(f, "{:.2}°C", temp),
            TemperatureUnit::Fahrenheit(temp) => write!(f, "{:.2}°F", temp),
            TemperatureUnit::Kelvin(temp) => write!(f, "{:.2}K", temp),
        }
    }
}

#[cfg(test)]
mod temperature_tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let celsius = TemperatureUnit::Celsius(0.0);
        assert_eq!(celsius.to_fahrenheit(), 32.0);
        
        let celsius = TemperatureUnit::Celsius(100.0);
        assert_eq!(celsius.to_fahrenheit(), 212.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let fahrenheit = TemperatureUnit::Fahrenheit(32.0);
        assert_eq!(fahrenheit.to_celsius(), 0.0);
    }

    #[test]
    fn test_kelvin_conversions() {
        let kelvin = TemperatureUnit::Kelvin(273.15);
        assert!((kelvin.to_celsius() - 0.0).abs() < 0.001);
    }
}
