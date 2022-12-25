use std::{
    collections::HashMap,
    error::Error,
    ops::{Add, AddAssign, SubAssign},
    time::Instant,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace1},
    multi::separated_list1,
    IResult,
};
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let blueprints = parse(&input);
    let minutes = 32;

    let res: u64 = blueprints[0..3]
        .into_par_iter()
        .map(|bp| {
            let mut robots = Resource::default();
            robots.ore = 1;
            let resources = Resource::default();
            let mut cache = HashMap::new();
            let curr_max = &mut 0;
            let max_geode = find_most_geodes(*bp, robots, resources, minutes, curr_max, &mut cache);
            dbg!(max_geode);
            max_geode as u64
        })
        .product();

    dbg!(res);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn find_most_geodes(
    bp: Blueprint,
    robots: Resource,
    resources: Resource,
    minutes: u16,
    curr_max: &mut u16,
    cache: &mut HashMap<(Blueprint, Resource, Resource, u16), u16>,
) -> u16 {
    // Branch cannot surpass current best solution.
    let m = robots.geode;
    let n = robots.geode + (minutes);
    let possible_max = (m..n).sum::<u16>() + resources.geode;
    if possible_max < *curr_max {
        return 0;
    }

    // Cache
    let state = (bp, robots, resources, minutes);
    if cache.contains_key(&state) {
        return *cache.get(&state).unwrap();
    }

    // Base case
    if minutes <= 0 {
        return resources.geode;
    }

    let mut max_geodes = [0; 5];

    // Buy geode robot.
    if bp.geode_robot <= resources {
        let mut resources = resources;
        let mut robots = robots;
        resources -= bp.geode_robot;
        resources += robots;
        robots.geode += 1;
        max_geodes[3] += find_most_geodes(bp, robots, resources, minutes - 1, curr_max, cache);
    }

    // Buy obsidian robot.
    let max_obsidian_robots = bp
        .ore_robot
        .obsidian
        .max(bp.clay_robot.obsidian)
        .max(bp.obsidian_robot.obsidian)
        .max(bp.geode_robot.obsidian);
    if bp.obsidian_robot <= resources && robots.obsidian < max_obsidian_robots {
        let mut resources = resources;
        let mut robots = robots;
        resources -= bp.obsidian_robot;
        resources += robots;
        robots.obsidian += 1;
        max_geodes[2] += find_most_geodes(bp, robots, resources, minutes - 1, curr_max, cache);
    }

    // Buy clay robot.
    let max_clay_robots = bp
        .ore_robot
        .clay
        .max(bp.clay_robot.clay)
        .max(bp.obsidian_robot.clay)
        .max(bp.geode_robot.clay);
    if bp.clay_robot <= resources && robots.clay < max_clay_robots {
        let mut res = resources;
        let mut rob = robots;
        res -= bp.clay_robot;
        res += rob;
        rob.clay += 1;
        max_geodes[1] += find_most_geodes(bp, rob, res, minutes - 1, curr_max, cache);
    }

    // Buy ore robot.
    let max_ore_robots = bp
        .ore_robot
        .ore
        .max(bp.clay_robot.ore)
        .max(bp.obsidian_robot.ore)
        .max(bp.geode_robot.ore);
    if bp.ore_robot <= resources && robots.ore < max_ore_robots {
        let mut resources = resources;
        let mut robots = robots;
        resources -= bp.ore_robot;
        resources += robots;
        robots.ore += 1;
        max_geodes[0] += find_most_geodes(bp, robots, resources, minutes - 1, curr_max, cache);
    }

    // Don't buy a robot.
    {
        let mut resources = resources;
        let robots = robots;
        resources += robots;
        max_geodes[4] += find_most_geodes(bp, robots, resources, minutes - 1, curr_max, cache);
    }

    let max_geode = max_geodes.into_iter().max().unwrap();
    cache.insert(state, max_geode);
    if max_geode > *curr_max {
        *curr_max = max_geode;
    }
    return max_geode;
}

fn parse_resource(input: &str) -> IResult<&str, (&str, u16)> {
    let (input, amount) = nom::character::complete::u16(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, (name, amount)))
}

fn parse_cost(input: &str) -> IResult<&str, Resource> {
    let (input, _) = multispace1(input)?;
    let (input, _) = separated_list1(multispace1, alpha1)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, resources) = separated_list1(tag(" and "), parse_resource)(input)?;
    let resources: HashMap<&str, u16> = resources.into_iter().collect();
    let (input, _) = tag(".")(input)?;

    let resource = Resource {
        ore: *resources.get("ore").unwrap_or(&0),
        clay: *resources.get("clay").unwrap_or(&0),
        obsidian: *resources.get("obsidian").unwrap_or(&0),
        geode: *resources.get("geode").unwrap_or(&0),
    };

    Ok((input, resource))
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = alpha1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, ore_robot) = parse_cost(input)?;
    let (input, clay_robot) = parse_cost(input)?;
    let (input, obsidian_robot) = parse_cost(input)?;
    let (input, geode_robot) = parse_cost(input)?;

    let blueprint = Blueprint {
        ore_robot,
        clay_robot,
        obsidian_robot,
        geode_robot,
    };

    Ok((input, blueprint))
}

fn parse(input: &str) -> Vec<Blueprint> {
    let (_, blueprints) = separated_list1(tag("\n"), parse_blueprint)(input).unwrap();
    blueprints
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Resource {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Blueprint {
    ore_robot: Resource,
    clay_robot: Resource,
    obsidian_robot: Resource,
    geode_robot: Resource,
}

impl Add for Resource {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Resource {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Resource {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl SubAssign for Resource {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl PartialOrd for Resource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use core::cmp::Ordering::*;
        match (
            self.ore.partial_cmp(&other.ore),
            self.clay.partial_cmp(&other.clay),
            self.obsidian.partial_cmp(&other.obsidian),
            self.geode.partial_cmp(&other.geode),
        ) {
            (Some(Equal), Some(Equal), Some(Equal), Some(Equal)) => Some(Equal),
            (Some(Less | Equal), Some(Less | Equal), Some(Less | Equal), Some(Less | Equal)) => {
                Some(Less)
            }
            (
                Some(Equal | Greater),
                Some(Equal | Greater),
                Some(Equal | Greater),
                Some(Equal | Greater),
            ) => Some(Greater),
            _ => None,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        use core::cmp::Ordering::*;

        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        use core::cmp::Ordering::*;

        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        use core::cmp::Ordering::*;

        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        use core::cmp::Ordering::*;

        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}
