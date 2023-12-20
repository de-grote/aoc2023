use crate::prelude::*;

#[derive(Clone, Debug)]
enum Module<'a> {
    Broadcaster(Vec<&'a str>),
    FlipFlop(Vec<&'a str>, bool),
    Conjuction(Vec<&'a str>, BTreeMap<&'a str, bool>),
}

pub fn part1(input: &str) -> Answer {
    let (_, mut map) = parse(input)?;

    for (name, v) in map.clone() {
        let v = match v {
            Module::Broadcaster(v) => v,
            Module::FlipFlop(v, _) => v,
            Module::Conjuction(v, _) => v,
        };
        for c in v {
            if let Some(Module::Conjuction(_, m)) = map.get_mut(c) {
                m.insert(name, false);
            }
        }
    }

    // dbg!(&map);

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _cycle in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", false));
        low_pulses += 1;
        while let Some((from, to, high)) = queue.pop_front() {
            match map.get_mut(to) {
                Some(Module::Broadcaster(v)) => {
                    low_pulses += v.len();
                    queue.extend(v.iter().map(|&x| (to, x, high)));
                }
                Some(Module::FlipFlop(v, b)) => {
                    if !high {
                        *b = !*b;
                        match b {
                            true => high_pulses += v.len(),
                            false => low_pulses += v.len(),
                        };
                        queue.extend(v.iter().map(|&x| (to, x, *b)));
                    }
                }
                Some(Module::Conjuction(v, m)) => {
                    let b = m.get_mut(from).unwrap();
                    *b = high;
                    let all_high = m.values().all(|&x| x);
                    match all_high {
                        true => low_pulses += v.len(),
                        false => high_pulses += v.len(),
                    }
                    queue.extend(v.iter().map(|&x| (to, x, !all_high)));
                }
                None => (),
            }
        }
    }

    dbg!(low_pulses, high_pulses);

    Ok((low_pulses * high_pulses).to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, mut map) = parse(input)?;

    for (name, v) in map.clone() {
        let v = match v {
            Module::Broadcaster(v) => v,
            Module::FlipFlop(v, _) => v,
            Module::Conjuction(v, _) => v,
        };
        for c in v {
            if let Some(Module::Conjuction(_, m)) = map.get_mut(c) {
                m.insert(name, false);
            }
        }
    }

    // assumes the last thing to send a signal to rx is a conjunction module, if that's not true for your input, go write your own code
    let weird_last_conjuction_thing = *map
        .iter()
        .find_map(|(name, module)| {
            if let Module::Conjuction(v, _) = module {
                v.contains(&"rx").then_some(name)
            } else {
                None
            }
        })
        .unwrap();
    let amount_of_stuff_pointing_to_the_weird_last_conjunction_thing = map
        .iter()
        .filter(|(_, v)| {
            let v = match v {
                Module::Broadcaster(v) => v,
                Module::FlipFlop(v, _) => v,
                Module::Conjuction(v, _) => v,
            };
            v.contains(&weird_last_conjuction_thing)
        })
        .count();
    let mut random_map_to_check_a_few_thingies: BTreeMap<&str, u64> = BTreeMap::new();

    for cycle in 1.. {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", false));
        while let Some((from, to, high)) = queue.pop_front() {
            match map.get_mut(to) {
                Some(Module::Broadcaster(v)) => {
                    queue.extend(v.iter().map(|&x| (to, x, high)));
                }
                Some(Module::FlipFlop(v, b)) => {
                    if !high {
                        *b = !*b;
                        queue.extend(v.iter().map(|&x| (to, x, *b)));
                    }
                }
                Some(Module::Conjuction(v, m)) => {
                    let b = m.get_mut(from).unwrap();
                    *b = high;

                    if to == weird_last_conjuction_thing {
                        for (key, _) in m.clone().into_iter().filter(|x| x.1) {
                            let k = random_map_to_check_a_few_thingies.entry(key);
                            k.or_insert(cycle);
                            if random_map_to_check_a_few_thingies.len()
                                == amount_of_stuff_pointing_to_the_weird_last_conjunction_thing
                            {
                                let result = random_map_to_check_a_few_thingies
                                    .into_iter()
                                    .map(|x| x.1)
                                    .fold(1, lcm);
                                return Ok(result.to_string());
                            }
                        }
                    }

                    let all_high = m.values().all(|&x| x);
                    queue.extend(v.iter().map(|&x| (to, x, !all_high)));
                }
                // rx
                None => {
                    if !high {
                        // prob never gonna happen
                        return Ok(cycle.to_string());
                    }
                }
            }
        }
    }

    panic!("couldn't find answer")
}

fn parse(input: &str) -> IResult<&str, BTreeMap<&str, Module>> {
    let mut it = iterator(
        input,
        terminated(
            separated_pair(
                tuple((alt((tag("&"), tag("%"), tag(""))), alpha1)),
                tag(" -> "),
                separated_list1(tag(", "), alpha1),
            ),
            multispace0,
        ),
    );
    let modules = it
        .map(|((t, name), strs)| {
            (
                name,
                match t {
                    "" => Module::Broadcaster(strs),
                    "%" => Module::FlipFlop(strs, false),
                    "&" => Module::Conjuction(strs, BTreeMap::new()),
                    x => panic!("{}", x),
                },
            )
        })
        .collect();
    let (input, _) = it.finish()?;
    Ok((input, modules))
}
