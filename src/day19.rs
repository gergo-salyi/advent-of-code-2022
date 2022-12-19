use nom::{
    bytes::complete::take_till,
    character::{
        complete::{u8 as nom_u8, u16 as nom_u16}, 
        is_digit
    },
    multi::{fold_many1, fold_many_m_n},
    sequence::preceded,
    IResult,
};

const INPUT: &[u8] = include_bytes!("../res/input19");

#[allow(unused)]
pub fn part1() {
    let ans = run1(INPUT);
    assert_eq!(ans, 1981);
    println!("{}", ans)
}

#[allow(unused)]
pub fn part2() {
    let ans = run2(INPUT);
    assert_eq!(ans, 10962); // 58 * 9 * 21
    println!("{}", ans)
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_robot_ore: u8,
    clay_robot_ore: u8,
    obs_robot_ore: u8,
    obs_robot_clay: u8,
    geo_robot_ore: u8,
    geo_robot_obs: u8
}

fn preceded_decimal(input: &[u8]) -> IResult<&[u8], u8> {
    preceded(take_till(is_digit), nom_u8)(input)
}

fn blueprint(input: &[u8]) -> IResult<&[u8], Blueprint> {
    let (input, id)             = preceded_decimal(input)?;
    let (input, ore_robot_ore)  = preceded_decimal(input)?;
    let (input, clay_robot_ore) = preceded_decimal(input)?;
    let (input, obs_robot_ore)  = preceded_decimal(input)?;
    let (input, obs_robot_clay) = preceded_decimal(input)?;
    let (input, geo_robot_ore)  = preceded_decimal(input)?;
    let (input, geo_robot_obs)  = preceded_decimal(input)?;
    
    Ok((input, Blueprint {
        id,
        ore_robot_ore,
        clay_robot_ore,
        obs_robot_ore,
        obs_robot_clay,
        geo_robot_ore,
        geo_robot_obs
    }))
}

fn parse(input: &[u8]) -> IResult<&[u8], Vec<Blueprint>> {
    fold_many1(
        blueprint,
        || Vec::with_capacity(input.len() / 159),
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        }
    )(input)
}

#[derive(Clone, Debug)]
struct State {
    time_left: u8,

    ore_robot: u8,
    clay_robot: u8,
    obsi_robot: u8,
    geo_robot: u8,

    ore: u8,
    clay: u8,
    obsi: u8, 
    geo: u8
}

fn push_next_states(
    queue: &mut Vec<State>,
    max_geode: &mut u8,
    state: &State, 
    blueprint: &Blueprint
) {
    if state.time_left == 1 {
        *max_geode = (state.geo + state.geo_robot).max(*max_geode);
        return;
    }

    let expected_max_geode = state.geo as u64 + (
        state.time_left as u64 * (state.geo_robot + state.time_left - 1) as u64);
    
    if expected_max_geode <= *max_geode as u64 {
        return;
    }
    
    let queue_start_len = queue.len();

    // Build geode robot
    if state.ore >= blueprint.geo_robot_ore 
        && state.obsi >= blueprint.geo_robot_obs 
    {
        queue.push(State { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot,
            clay_robot: state.clay_robot,
            obsi_robot: state.obsi_robot,
            geo_robot: state.geo_robot + 1,

            ore: state.ore - blueprint.geo_robot_ore + state.ore_robot,
            clay: state.clay + state.clay_robot,
            obsi: state.obsi - blueprint.geo_robot_obs + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });

        // assume building the geode robot is a must
        return;
    }
    
    // Build obsidian robot
    if state.time_left > 2 && state.ore >= blueprint.obs_robot_ore 
        && state.clay >= blueprint.obs_robot_clay 
    {
        queue.push(State { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot,
            clay_robot: state.clay_robot,
            obsi_robot: state.obsi_robot + 1,
            geo_robot: state.geo_robot,

            ore: state.ore - blueprint.obs_robot_ore + state.ore_robot,
            clay: state.clay - blueprint.obs_robot_clay + state.clay_robot,
            obsi: state.obsi + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });

        // assume building the obsidian robot is a must
        return;
    }

    // Build ore robot
    if state.time_left > 4 && state.ore >= blueprint.ore_robot_ore {
        queue.push(State { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot + 1,
            clay_robot: state.clay_robot,
            obsi_robot: state.obsi_robot,
            geo_robot: state.geo_robot,

            ore: state.ore - blueprint.ore_robot_ore + state.ore_robot,
            clay: state.clay + state.clay_robot,
            obsi: state.obsi + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });
    }

    // Build clay robot
    if state.time_left > 3 && state.ore >= blueprint.clay_robot_ore {
        queue.push(State { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot,
            clay_robot: state.clay_robot + 1,
            obsi_robot: state.obsi_robot,
            geo_robot: state.geo_robot,

            ore: state.ore - blueprint.clay_robot_ore + state.ore_robot,
            clay: state.clay + state.clay_robot,
            obsi: state.obsi + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });
    }

    // Build nothing (if not every other things could be built)
    if queue.len() - queue_start_len < 3 {

        queue.push(State { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot,
            clay_robot: state.clay_robot,
            obsi_robot: state.obsi_robot,
            geo_robot: state.geo_robot,

            ore: state.ore + state.ore_robot,
            clay: state.clay + state.clay_robot,
            obsi: state.obsi + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });
    }

}

pub fn run1(input: &[u8]) -> u64 {
    let blueprints = parse(input).unwrap().1;
    let mut quality_levels: Vec<u64> = Vec::with_capacity(blueprints.len());

    let init = State {
        time_left: 24,
        ore_robot: 1,
        clay_robot: 0,
        obsi_robot: 0,
        geo_robot: 0,
        ore: 0,
        clay: 0,
        obsi: 0, 
        geo: 0
    };

    let mut queue = Vec::with_capacity(24 * 5);

    for blueprint in &blueprints {

        println!("Blueprint: {}", blueprint.id);

        queue.push(init.clone());
        let mut max_geode = 0u8;

        while let Some(state) = queue.pop() {
            push_next_states(&mut queue, &mut max_geode, &state, blueprint)
        }

        quality_levels.push(blueprint.id as u64 * max_geode as u64)
    }

    quality_levels.iter().sum()
}



#[derive(Debug)]
struct Blueprint2 {
    id: u16,
    ore_robot_ore: u16,
    clay_robot_ore: u16,
    obs_robot_ore: u16,
    obs_robot_clay: u16,
    geo_robot_ore: u16,
    geo_robot_obs: u16
}

fn preceded_decimal2(input: &[u8]) -> IResult<&[u8], u16> {
    preceded(take_till(is_digit), nom_u16)(input)
}

fn blueprint2(input: &[u8]) -> IResult<&[u8], Blueprint2> {
    let (input, id)             = preceded_decimal2(input)?;
    let (input, ore_robot_ore)  = preceded_decimal2(input)?;
    let (input, clay_robot_ore) = preceded_decimal2(input)?;
    let (input, obs_robot_ore)  = preceded_decimal2(input)?;
    let (input, obs_robot_clay) = preceded_decimal2(input)?;
    let (input, geo_robot_ore)  = preceded_decimal2(input)?;
    let (input, geo_robot_obs)  = preceded_decimal2(input)?;
    
    Ok((input, Blueprint2 {
        id,
        ore_robot_ore,
        clay_robot_ore,
        obs_robot_ore,
        obs_robot_clay,
        geo_robot_ore,
        geo_robot_obs
    }))
}

fn parse2(input: &[u8]) -> IResult<&[u8], Vec<Blueprint2>> {
    fold_many_m_n(
        1,
        3,
        blueprint2,
        || Vec::with_capacity(input.len() / 159),
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        }
    )(input)
}

#[derive(Clone, Debug)]
struct State2 {
    time_left: u16,

    ore_robot: u16,
    clay_robot: u16,
    obsi_robot: u16,
    geo_robot: u16,

    ore: u16,
    clay: u16,
    obsi: u16, 
    geo: u16
}

fn push_next_states2(
    queue: &mut Vec<State2>,
    max_geode: &mut u16,
    state: &State2, 
    blueprint: &Blueprint2
) {
    if state.time_left == 1 {
        *max_geode = (state.geo + state.geo_robot).max(*max_geode);
        return;
    }

    let expected_max_geode = state.geo as u64 + (
        state.time_left as u64 * (state.geo_robot + state.time_left - 1) as u64);
    
    if expected_max_geode <= *max_geode as u64 {
        return;
    }
    
    let queue_start_len = queue.len();

    // Build geode robot
    if state.ore >= blueprint.geo_robot_ore 
        && state.obsi >= blueprint.geo_robot_obs 
    {
        queue.push(State2 { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot,
            clay_robot: state.clay_robot,
            obsi_robot: state.obsi_robot,
            geo_robot: state.geo_robot + 1,

            ore: state.ore - blueprint.geo_robot_ore + state.ore_robot,
            clay: state.clay + state.clay_robot,
            obsi: state.obsi - blueprint.geo_robot_obs + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });

        // assume building the geode robot is a must
        return;
    }

    // Build obsidian robot
    if state.time_left > 2 && state.ore >= blueprint.obs_robot_ore 
        && state.clay >= blueprint.obs_robot_clay 
    {
        queue.push(State2 { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot,
            clay_robot: state.clay_robot,
            obsi_robot: state.obsi_robot + 1,
            geo_robot: state.geo_robot,

            ore: state.ore - blueprint.obs_robot_ore + state.ore_robot,
            clay: state.clay - blueprint.obs_robot_clay + state.clay_robot,
            obsi: state.obsi + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });
    }
    
    // Build clay robot
    if state.time_left > 3 && state.ore >= blueprint.clay_robot_ore {
        queue.push(State2 { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot,
            clay_robot: state.clay_robot + 1,
            obsi_robot: state.obsi_robot,
            geo_robot: state.geo_robot,

            ore: state.ore - blueprint.clay_robot_ore + state.ore_robot,
            clay: state.clay + state.clay_robot,
            obsi: state.obsi + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });
    }

    // Build ore robot
    if state.time_left > 4 && state.ore >= blueprint.ore_robot_ore {
        queue.push(State2 { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot + 1,
            clay_robot: state.clay_robot,
            obsi_robot: state.obsi_robot,
            geo_robot: state.geo_robot,

            ore: state.ore - blueprint.ore_robot_ore + state.ore_robot,
            clay: state.clay + state.clay_robot as u16,
            obsi: state.obsi + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });
    }

    // Build nothing (if not every other things could be built)
    if queue.len() - queue_start_len < 3 {

        queue.push(State2 { 
            time_left: state.time_left - 1,

            ore_robot: state.ore_robot,
            clay_robot: state.clay_robot,
            obsi_robot: state.obsi_robot,
            geo_robot: state.geo_robot,

            ore: state.ore + state.ore_robot,
            clay: state.clay + state.clay_robot,
            obsi: state.obsi + state.obsi_robot,
            geo: state.geo + state.geo_robot,
        });
    }

}

pub fn run2(input: &[u8]) -> u64 {
    let blueprints = parse2(input).unwrap().1;

    let mut max_geodes: Vec<u64> = Vec::with_capacity(blueprints.len());

    let init = State2 {
        time_left: 32,
        ore_robot: 1,
        clay_robot: 0,
        obsi_robot: 0,
        geo_robot: 0,
        ore: 0,
        clay: 0,
        obsi: 0, 
        geo: 0
    };

    let mut queue = Vec::with_capacity(32 * 5);

    for blueprint in &blueprints {

        println!("Blueprint: {}", blueprint.id);

        queue.push(init.clone());
        let mut max_geode = 0u16;

        while let Some(state) = queue.pop() {
            // if queue.len() == 0 { println!("{state:#?}"); }
            push_next_states2(&mut queue, &mut max_geode, &state, blueprint)
        }
        
        println!("max geode: {}", max_geode);
        println!();

        max_geodes.push(max_geode as u64)
    }

    max_geodes.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example19");

    #[test]
    #[ignore]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 33)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 3472)
    }
}
