use std::collections::HashMap;
use std::ops::Range;
use itertools::Itertools;

#[test]
fn part_one() {
    let lines = include_str!("../05.input").trim().lines().collect_vec();
    let almanac = Almanac::parse_input(&lines);
    let seeds = parse_seeds_from_input(&lines);
    let location =
        seeds.iter()
            .map(|seed| almanac.seed_to_location_pipeline(seed))
            .min()
            .unwrap();
    println!("Day 5: If You Give A Seed A Fertilizer is {}", location)
}

#[test]
fn part_one_example() {
    let lines =
        r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    let lines = lines.trim().lines().collect_vec();
    let almanac = Almanac::parse_input(&lines);
    let seeds = parse_seeds_from_input(&lines);
    let location =
        seeds.iter()
            .map(|seed| almanac.seed_to_location_pipeline(seed))
            .min()
            .unwrap();
    println!("Day 5: If You Give A Seed A Fertilizer is {}", location)
}

#[test]
fn part_two() {}

struct ElvishMapper {
    dest_range: Range<u128>,
    source_range: Range<u128>,
}

impl ElvishMapper {
    fn parse(mapper: &str) -> Self {
        let mapper =
            mapper.split_whitespace()
                .filter_map(|x| x.parse::<u128>().ok())
                .collect_vec();
        let (min_dest, min_source, len) = (mapper[0], mapper[1], mapper[2]);
        let max_dest = min_dest + len;
        let max_source = min_source + len;
        Self {
            dest_range: min_dest..max_dest,
            source_range: min_source..max_source,
        }
    }

    fn map(&self, source: &u128) -> Option<u128> {
        if self.source_range.contains(source) {
            let shift = source - self.source_range.start;
            let dest = self.dest_range.start + shift;
            Some(dest)
        } else {
            None
        }
    }
}

struct ElvishMapperCollection {
    mappers: Vec<ElvishMapper>,
}

impl ElvishMapperCollection {
    fn new() -> Self {
        Self {
            mappers: vec![]
        }
    }

    fn append_mapper(&mut self, mapper: ElvishMapper) {
        self.mappers.push(mapper)
    }

    fn map(&self, source: &u128) -> u128 {
        for mapper in &self.mappers {
            if let Some(dest) = mapper.map(source) {
                return dest;
            }
        }
        *source
    }
}

struct Almanac {
    named_collections: HashMap<String, ElvishMapperCollection>,
}

impl Almanac {
    fn parse_input(input: &Vec<&str>) -> Self {
        let mut named_collections = HashMap::new();
        let mut buf = vec![];
        for l in input.iter().skip(2) {
            if l.is_empty() {
                Almanac::parse_collection(&mut named_collections, &mut buf);
                continue;
            }
            buf.push(l);
        }
        Almanac::parse_collection(&mut named_collections, &mut buf);
        Self { named_collections }
    }

    fn map(&self, chapter: &str, source: &u128) -> u128 {
        self.named_collections.get(chapter)
            .unwrap()
            .map(source)
    }

    fn parse_collection(map: &mut HashMap<String, ElvishMapperCollection>, buf: &mut Vec<&str>) {
        if buf.is_empty() {
            return;
        }
        let mut collection = ElvishMapperCollection::new();
        for l in buf.iter().skip(1) {
            let mapper = ElvishMapper::parse(l);
            collection.append_mapper(mapper)
        }
        let name = String::from(buf[0]);
        map.insert(name, collection);
        buf.clear();
    }

    fn seed_to_location_pipeline(&self, seed: &u128) -> u128 {
        println!("--- seed_to_location_pipeline --\n");

        let soil = self.map("seed-to-soil map:", seed);
        println!("seed-to-soil            {} to {}", seed, soil);

        let fertilizer = self.map("soil-to-fertilizer map:", &soil);
        println!("soil-to-fertilizer      {} to {}", soil, fertilizer);

        let water = self.map("fertilizer-to-water map:", &fertilizer);
        println!("fertilizer-to-water     {} to {}", fertilizer, water);

        let light = self.map("water-to-light map:", &water);
        println!("water-to-light          {} to {}", water, light);

        let temperature = self.map("light-to-temperature map:", &light);
        println!("light-to-temperature    {} to {}", light, temperature);

        let humidity = self.map("temperature-to-humidity map:", &temperature);
        println!("temperature-to-humidity {} to {}", temperature, humidity);

        let location = self.map("humidity-to-location map:", &humidity);
        println!("humidity-to-location    {} to {}\n", humidity, location);

        location
    }
}

fn parse_seeds_from_input(input: &Vec<&str>) -> Vec<u128> {
    let (_, line) = input[0].split_once(": ").unwrap();
    line.split_whitespace()
        .map(|x| x.parse())
        .map(Result::unwrap)
        .collect_vec()
}