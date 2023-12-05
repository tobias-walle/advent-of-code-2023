use anyhow::{Context, Result};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{multispace1, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
};
use utils::{
    parsing::{self, parse_with_nom},
    read_input_file_as_string,
};

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<usize> {
    let problem = parse(input)?;
    let result = problem
        .starting_seeds
        .iter()
        .map(|seed| {
            let soil = problem.seed_to_soil_map.get_destination(seed);
            let fertilizer = problem.soil_to_fertilizer_map.get_destination(&soil);
            let water = problem.fertilizer_to_water_map.get_destination(&fertilizer);
            let light = problem.water_to_light_map.get_destination(&water);
            let temperature = problem.light_to_temperature_map.get_destination(&light);
            let humidity = problem
                .temperature_to_humidity_map
                .get_destination(&temperature);
            problem.humidity_to_location_map.get_destination(&humidity)
        })
        .min()
        .expect("No starting seeds found");
    dbg!(&result);
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub starting_seeds: Vec<usize>,
    pub seed_to_soil_map: Map,
    pub soil_to_fertilizer_map: Map,
    pub fertilizer_to_water_map: Map,
    pub water_to_light_map: Map,
    pub light_to_temperature_map: Map,
    pub temperature_to_humidity_map: Map,
    pub humidity_to_location_map: Map,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub ranges: Vec<MapRange>,
}

impl Map {
    pub fn get_destination(&self, source: &usize) -> usize {
        for range in &self.ranges {
            if let Some(result) = range.get_destination(source) {
                return result;
            }
        }
        *source
    }

    pub fn get_source(&self, destination: &usize) -> usize {
        for range in &self.ranges {
            if let Some(result) = range.get_source(destination) {
                return result;
            }
        }
        *destination
    }
}

#[derive(Debug, Clone)]
pub struct MapRange {
    pub destination_range_start: usize,
    pub source_range_start: usize,
    pub range_length: usize,
}

impl MapRange {
    pub fn get_destination(&self, source: &usize) -> Option<usize> {
        let source_range = self.source_range_start..(self.source_range_start + self.range_length);
        if source_range.contains(source) {
            let offset = source - self.source_range_start;
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }
    pub fn get_source(&self, destination: &usize) -> Option<usize> {
        let destination_range =
            self.destination_range_start..(self.destination_range_start + self.range_length);
        if destination_range.contains(destination) {
            let offset = destination - self.destination_range_start;
            Some(self.source_range_start + offset)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Result<Problem> {
    let parse_seeds = preceded(
        tuple((tag("seeds:"), space0)),
        separated_list1(space1, parsing::number),
    );
    let parse_map_range = map(
        tuple((
            preceded(space0, parsing::number),
            preceded(space0, parsing::number),
            preceded(space0, parsing::number),
        )),
        |(destination_range_start, source_range_start, range_length)| MapRange {
            destination_range_start,
            source_range_start,
            range_length,
        },
    );
    let parse_map = map(
        preceded(
            tuple((take_until("map:"), tag("map:"), multispace1)),
            separated_list1(multispace1, parse_map_range),
        ),
        |ranges| Map { ranges },
    );
    let parse_problem = map(
        tuple((parse_seeds, separated_list1(multispace1, parse_map))),
        |(starting_seeds, maps)| Problem {
            starting_seeds,
            seed_to_soil_map: maps[0].clone(),
            soil_to_fertilizer_map: maps[1].clone(),
            fertilizer_to_water_map: maps[2].clone(),
            water_to_light_map: maps[3].clone(),
            light_to_temperature_map: maps[4].clone(),
            temperature_to_humidity_map: maps[5].clone(),
            humidity_to_location_map: maps[6].clone(),
        },
    );
    let problem = parse_with_nom(input.trim(), parse_problem)?;
    Ok(problem)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 35);
    }
}
