use std::iter::Iterator;
use std::convert::TryInto;

/// Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
/// required for a module, take its mass, divide by three, round down, and subtract 2.
pub fn fuel_required(mass: u32) -> i32 {
    let mass: i32 = mass.try_into().unwrap();
    mass / 3 - 2
}

pub fn fuel_mass(fuel: u32) -> u32 {
    let m = fuel_required(fuel.try_into().unwrap());
    if m <= 0 {
        return 0;
    }
    let m: u32 = m.try_into().unwrap();
    m + fuel_mass(m)
}

pub fn entry_a<I>(lines: I) -> i32
    where I: Iterator<Item=u32>
{
    let mut total_fuel: i32 = 0;
    for mass in lines {
        total_fuel += fuel_required(mass)
    }
    total_fuel
}

pub fn entry_b<I>(lines: I) -> u32
    where I: Iterator<Item=u32>
{
    let mut total_fuel_mass = 0;
    for mass in lines {
        total_fuel_mass += fuel_mass(mass);
    }
    total_fuel_mass
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_results_a() {
        assert_eq!(fuel_mass(12), 2);
        assert_eq!(fuel_mass(1969), 966);
        assert_eq!(fuel_mass(100756), 50346);
    }

    #[test]
    fn official_results_b() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }
}