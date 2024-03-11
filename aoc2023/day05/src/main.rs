use std::collections::HashMap;

use utils::files::read_lines;

struct MaterialMap {
    items: Vec<(u32, u32, u32)>,
}

struct Almonac {
    seeds: Vec<u32>,

    seed_soil: MaterialMap,
    soil_fetilizer: MaterialMap,
    fertilizer_water: MaterialMap,
    whater_light: MaterialMap,
    light_temperature: MaterialMap,
    temperature_humidity: MaterialMap,
    humidity_location: MaterialMap,
}

impl Almonac {
    fn make(lines: Vec<String>) -> Self {
        todo!()
    }

    fn lowest_location(&self) -> u32 {
        todo!()
    }
}

fn main() {
    let lines = read_lines("day05/input.txt").expect("Failed to read input");
    let alomanc = Almonac::make(lines.flatten().collect());
    println!("Part 1 = {}", alomanc.lowest_location());
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod day05_tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: Vec<&'static str> = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
    }

    #[test]
    fn test_input_parser() {}
}