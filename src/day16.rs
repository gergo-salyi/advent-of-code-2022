use ahash::{HashMap, HashMapExt};
use nom::{
    bytes::complete::{tag, take, take_till},
    character::complete::{u8 as nom_u8, multispace0},
    combinator::map,
    multi::separated_list1,
    IResult,
};

use std::collections::{hash_map::Entry, VecDeque};

const INPUT: &[u8] = include_bytes!("../res/input16");

#[allow(unused)]
pub fn part1() {
    println!("{}", run1(INPUT)) // 1873
}

#[allow(unused)]
pub fn part2() {
    println!("{}", run2(INPUT)) //
}

#[derive(Debug, Default, Clone)]
struct Valve {
    flow_rate: u8,
    tunnels: Vec<u8>,
}

#[derive(Debug)]
struct Valve2 {
    flow_rate: u8,

    //           idx, distance
    tunnels: Vec<(u8, u8)>,
}

fn id_from_name(name: &[u8], name_id_map: &mut HashMap<[u8; 2], u8>) -> u8 {
    let next_id = name_id_map.len() as u8;
    match name_id_map.entry(name.try_into().unwrap()) {
        Entry::Occupied(hit) => *hit.get(),
        Entry::Vacant(miss) => *miss.insert(next_id),
    }
}

fn tunnels<'a>(
    input: &'a [u8],
    name_id_map: &mut HashMap<[u8; 2], u8>,
) -> IResult<&'a [u8], Vec<u8>> {

    separated_list1(
        tag(", "),
        map(take(2usize), |name: &[u8]| id_from_name(name, name_id_map)),
    )(input)
}

fn line<'a>(
    input: &'a [u8],
    name_id_map: &mut HashMap<[u8; 2], u8>,
) -> IResult<&'a [u8], (u8, Valve)> {

    let (input, _) = take(6usize)(input)?;
    let (input, id) = map(
        take(2usize), 
        |name: &[u8]| id_from_name(name, name_id_map)
    )(input)?;
    let (input, _) = take(15usize)(input)?;
    let (input, flow_rate) = nom_u8(input)?;
    let (input, _) = take_till(|b| (b'A'..=b'Z').contains(&b))(input)?;
    let (input, tunnels) = tunnels(input, name_id_map)?;
    let (input, _) = multispace0(input)?;

    let valve = Valve { flow_rate, tunnels };

    Ok((input, (id, valve)))
}

fn parse(input: &[u8]) -> (Vec<Valve>, Vec<u8>, u8) {

    let mut input = input;

    let valves_count_max = input.len() / 51;
    let mut name_id_map: HashMap<[u8; 2], u8> = HashMap::with_capacity(valves_count_max);
    // let mut valves = Vec::with_capacity(valves_count_max);
    let mut valves = vec![Valve::default(); valves_count_max];
    let mut non_zero_flow_ids = Vec::with_capacity(valves_count_max);

    while let Ok((i, (id, valve))) = line(input, &mut name_id_map) {
        input = i;
        if valve.flow_rate > 0 {
            non_zero_flow_ids.push(id);
        }
        // unsafe {
        //     // assert!(valves_count_max > id as usize);
        //     *valves.get_unchecked_mut(id as usize) = valve;
        // }
        valves[id as usize] = valve;
    }

    // This parser minimizes the copying but
    // is unsafe againt incorrect input
    // unsafe {
    //     valves.set_len(name_id_map.len());
    // }
    valves.truncate(name_id_map.len());

    // println!("{:?}", name_id_map.iter().map(|(k,v)| (std::str::from_utf8(k).unwrap(),v)).collect::<Vec<_>>() );

    let start = name_id_map.get(b"AA").unwrap();

    (valves, non_zero_flow_ids, *start)
}

fn distances(valves: &Vec<Valve>, a: u8, queue: &mut VecDeque<u8>) -> Vec<u8> {
    let mut dists = vec![u8::MAX; valves.len()];
    queue.push_back(a);
    dists[a as usize] = 0;

    while let Some(c) = queue.pop_front() {
        let next_dist = dists[c as usize] + 1;
        for &tunnel in valves[c as usize].tunnels.iter() {
            if dists[tunnel as usize] > next_dist {
                dists[tunnel as usize] = next_dist;
                queue.push_back(tunnel);
            }
        }
    }

    dists
}

#[derive(Debug)]
struct State {
    current_valve2_idx: u8,
    time_left: u8,
    valve2_idx_to_open: Vec<u8>,
    total_eventual_p_rel: u64
}

pub fn run1(input: &[u8]) -> u64 {
    let mut answer = 0u64;

    let (valves, non_zero_flow_ids, start) = parse(input);

    //                                                           + start
    let mut valves2 = Vec::with_capacity(non_zero_flow_ids.len() + 1);

    let mut queue = VecDeque::with_capacity(valves.len());
    let mut dists = vec![u8::MAX; valves.len()];

    for (new_idx, &idx) in non_zero_flow_ids.iter().enumerate() {

        dists.fill(u8::MAX);
        queue.push_back(idx);
        dists[idx as usize] = 0;

        while let Some(c) = queue.pop_front() {
            let next_dist = dists[c as usize] + 1;
            for &tunnel in valves[c as usize].tunnels.iter() {
                if dists[tunnel as usize] > next_dist {
                    dists[tunnel as usize] = next_dist;
                    queue.push_back(tunnel);
                }
            }
        }

        // dists: distances as Vec<idx>

        // let mut tunnels = Vec::with_capacity(non_zero_flow_ids.len() - 1);
        let mut tunnels = Vec::with_capacity(non_zero_flow_ids.len() - 1);

        for (other_new_idx, &other_idx) in non_zero_flow_ids
            .iter().enumerate()
        {
            // if other_new_idx != new_idx {
                tunnels.push((other_new_idx as u8, dists[other_idx as usize]));
            // }
        }

        valves2.push (Valve2 {
            flow_rate: valves[idx as usize].flow_rate,
            tunnels
        });
    }

    // add the starting point to valves2
    dists.fill(u8::MAX);
    queue.push_back(start);
    dists[start as usize] = 0;
    while let Some(c) = queue.pop_front() {
        let next_dist = dists[c as usize] + 1;
        for &tunnel in valves[c as usize].tunnels.iter() {
            if dists[tunnel as usize] > next_dist {
                dists[tunnel as usize] = next_dist;
                queue.push_back(tunnel);
            }
        }
    }
    let mut tunnels = Vec::with_capacity(non_zero_flow_ids.len());

    for (other_new_idx, &other_idx) in non_zero_flow_ids
        .iter().enumerate()
    {
        tunnels.push((other_new_idx as u8, dists[other_idx as usize]));
    }

    valves2.push (Valve2 {
        flow_rate: 0,
        tunnels
    });

    // get rid of mut
    let valves2 = valves2;

    // do the search on valves2

    let mut game_tree = VecDeque::new();

    let init = State {
        current_valve2_idx: valves2.len() as u8 - 1,
        time_left: 30,
        valve2_idx_to_open: (0..valves2.len() as u8 - 1).collect::<Vec<u8>>(),
        total_eventual_p_rel: 0
    };

    game_tree.push_back(init);

    while let Some(state) = game_tree.pop_front() {
        // println!("{state:?}");

        let current_valve = &valves2[state.current_valve2_idx as usize];

        // brench the game tree for every next valve we can move to
        for (idx_in_list_idx, &idx) in state.valve2_idx_to_open.iter().enumerate() {

            let tunnel = current_valve.tunnels[idx as usize];
            assert_eq!(tunnel.0, idx);

            let new_time = state.time_left as i8 - tunnel.1 as i8 - 1;

            if new_time < 0 {
                answer = answer.max(state.total_eventual_p_rel);
                continue;
            }

            let next_valve = &valves2[idx as usize];
            let gained_p_rel = next_valve.flow_rate as u64 * new_time as u64;

            let mut remaining_to_open = state.valve2_idx_to_open.clone();
            remaining_to_open.swap_remove(idx_in_list_idx);

            if remaining_to_open.is_empty() {
                answer = answer.max(state.total_eventual_p_rel + gained_p_rel);
                continue;
            }

            game_tree.push_back(State { 
                current_valve2_idx: idx, 
                time_left: new_time as u8, 
                valve2_idx_to_open: remaining_to_open, 
                total_eventual_p_rel: state.total_eventual_p_rel + gained_p_rel
            })
        }
    }

    answer
}


#[derive(Debug)]
struct State2 {
    is_elephant: bool,
    me_current_valve2_idx: u8,
    el_current_valve2_idx: u8,
    me_time_left: u8,
    el_time_left: u8,
    valve2_idx_to_open: Vec<u8>,
    total_eventual_p_rel: u64
}

pub fn run2(input: &[u8]) -> u64 {
    let mut answer = 0u64;

    let (valves, non_zero_flow_ids, start) = parse(input);

    //                                                           + start
    let mut valves2 = Vec::with_capacity(non_zero_flow_ids.len() + 1);

    let mut queue = VecDeque::with_capacity(valves.len());
    let mut dists = vec![u8::MAX; valves.len()];

    for (new_idx, &idx) in non_zero_flow_ids.iter().enumerate() {

        dists.fill(u8::MAX);
        queue.push_back(idx);
        dists[idx as usize] = 0;

        while let Some(c) = queue.pop_front() {
            let next_dist = dists[c as usize] + 1;
            for &tunnel in valves[c as usize].tunnels.iter() {
                if dists[tunnel as usize] > next_dist {
                    dists[tunnel as usize] = next_dist;
                    queue.push_back(tunnel);
                }
            }
        }

        // dists: distances as Vec<idx>

        // let mut tunnels = Vec::with_capacity(non_zero_flow_ids.len() - 1);
        let mut tunnels = Vec::with_capacity(non_zero_flow_ids.len() - 1);

        for (other_new_idx, &other_idx) in non_zero_flow_ids
            .iter().enumerate()
        {
            // if other_new_idx != new_idx {
                tunnels.push((other_new_idx as u8, dists[other_idx as usize]));
            // }
        }

        valves2.push (Valve2 {
            flow_rate: valves[idx as usize].flow_rate,
            tunnels
        });
    }

    // add the starting point to valves2
    dists.fill(u8::MAX);
    queue.push_back(start);
    dists[start as usize] = 0;
    while let Some(c) = queue.pop_front() {
        let next_dist = dists[c as usize] + 1;
        for &tunnel in valves[c as usize].tunnels.iter() {
            if dists[tunnel as usize] > next_dist {
                dists[tunnel as usize] = next_dist;
                queue.push_back(tunnel);
            }
        }
    }
    let mut tunnels = Vec::with_capacity(non_zero_flow_ids.len());

    for (other_new_idx, &other_idx) in non_zero_flow_ids
        .iter().enumerate()
    {
        tunnels.push((other_new_idx as u8, dists[other_idx as usize]));
    }

    valves2.push (Valve2 {
        flow_rate: 0,
        tunnels
    });

    // get rid of mut
    let valves2 = valves2;

    // do the search on valves2

    let mut game_tree = VecDeque::with_capacity(200_000_000);

    let init = State2 {
        is_elephant: false,
        me_current_valve2_idx: valves2.len() as u8 - 1,
        el_current_valve2_idx: valves2.len() as u8 - 1,
        me_time_left: 26,
        el_time_left: 26,
        valve2_idx_to_open: (0..valves2.len() as u8 - 1).collect::<Vec<u8>>(),
        total_eventual_p_rel: 0
    };

    game_tree.push_back(init);

    while let Some(state) = game_tree.pop_front() {
        // println!("{state:?}");

        if state.is_elephant {

            let current_valve = &valves2[state.el_current_valve2_idx as usize];

            // brench the game tree for every next valve we can move to
            for (idx_in_list_idx, &idx) in state.valve2_idx_to_open
                .iter().enumerate() 
            {
                let tunnel = current_valve.tunnels[idx as usize];
                assert_eq!(tunnel.0, idx);

                let new_time = state.el_time_left as i8 - tunnel.1 as i8 - 1;

                if new_time < 0 {
                    if state.me_time_left == 0 {
                        answer = answer.max(state.total_eventual_p_rel);
                        continue;
                    } else {
                        game_tree.push_back(State2 { 
                            is_elephant: false,
                            el_time_left: 0,
                            valve2_idx_to_open: state.valve2_idx_to_open.clone(),
                            ..state
                        });
                        continue;
                    }
                }

                let next_valve = &valves2[idx as usize];
                let gained_p_rel = next_valve.flow_rate as u64 * new_time as u64;

                let mut remaining_to_open = state.valve2_idx_to_open.clone();
                remaining_to_open.swap_remove(idx_in_list_idx);

                if remaining_to_open.is_empty() {
                    answer = answer.max(state.total_eventual_p_rel + gained_p_rel);
                    continue;
                }
                
                // pruning with dirty random hardcoded trial-and-error params
                if (new_time as u8) < 15 && state.el_time_left < 15 && (state.total_eventual_p_rel + gained_p_rel) < 1600 {continue}

                game_tree.push_back(State2 {
                    is_elephant: new_time as u8 > state.me_time_left,
                    me_current_valve2_idx: state.me_current_valve2_idx,
                    el_current_valve2_idx: idx,
                    me_time_left: state.me_time_left,
                    el_time_left: new_time as u8, 
                    valve2_idx_to_open: remaining_to_open, 
                    total_eventual_p_rel: state.total_eventual_p_rel + gained_p_rel
                })
            }

        } else {

            let current_valve = &valves2[state.me_current_valve2_idx as usize];

            // brench the game tree for every next valve we can move to
            for (idx_in_list_idx, &idx) in state.valve2_idx_to_open
                .iter().enumerate() 
            {
                let tunnel = current_valve.tunnels[idx as usize];
                assert_eq!(tunnel.0, idx);

                let new_time = state.me_time_left as i8 - tunnel.1 as i8 - 1;

                if new_time < 0 {
                    if state.el_time_left == 0 {
                        answer = answer.max(state.total_eventual_p_rel);
                        continue;
                    } else {
                        game_tree.push_back(State2 { 
                            is_elephant: true,
                            me_time_left: 0,
                            valve2_idx_to_open: state.valve2_idx_to_open.clone(),
                            ..state
                        });
                        continue;
                    }
                }

                let next_valve = &valves2[idx as usize];
                let gained_p_rel = next_valve.flow_rate as u64 * new_time as u64;

                let mut remaining_to_open = state.valve2_idx_to_open.clone();
                remaining_to_open.swap_remove(idx_in_list_idx);

                if remaining_to_open.is_empty() {
                    answer = answer.max(state.total_eventual_p_rel + gained_p_rel);
                    continue;
                }

                // pruning with dirty random hardcoded trial-and-error params
                if (new_time as u8) < 15 && state.el_time_left < 15 && (state.total_eventual_p_rel + gained_p_rel) < 1600 {continue}

                game_tree.push_back(State2 { 
                    is_elephant: state.el_time_left > new_time as u8,
                    me_current_valve2_idx: idx, 
                    el_current_valve2_idx: state.el_current_valve2_idx,
                    me_time_left: new_time as u8,
                    el_time_left: state.el_time_left,
                    valve2_idx_to_open: remaining_to_open, 
                    total_eventual_p_rel: state.total_eventual_p_rel + gained_p_rel
                })
            }

        }

    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = include_bytes!("../res/example16");

    #[test]
    fn test1() {
        assert_eq!(run1(EXAMPLE), 1651)
    }

    #[test]
    fn test2() {
        assert_eq!(run2(EXAMPLE), 1707)
    }
}
