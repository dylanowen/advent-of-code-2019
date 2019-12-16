use advent_of_code_2019::example;
use advent_of_code_2019::problem::{run, Problem, ProblemState, RunFor};
use env_logger::Env;
use regex::Regex;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

static ONE_TRILLION: usize = 1_000_000_000_000;

struct Fourteen {}

#[derive(Debug, Clone)]
struct Ingredient {
    cost: usize,
    material: String,
}

impl Ingredient {
    fn new(s: &str) -> Ingredient {
        let re = Regex::new(r"(\d+) (\w+)").unwrap();

        let parsed = re.captures(s.trim()).unwrap();

        let cost = parsed[1].parse::<usize>().expect("Parse error");

        Ingredient {
            cost,
            material: parsed[2].to_string(),
        }
    }
}

impl PartialEq for Ingredient {
    fn eq(&self, other: &Self) -> bool {
        self.material.eq(&other.material)
    }
}
impl Eq for Ingredient {}

impl Hash for Ingredient {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.material.hash(state)
    }
}

#[derive(Debug)]
struct Recipe {
    result: Ingredient,
    ingredients: Vec<Ingredient>,
}

impl Problem for Fourteen {
    type Input = HashMap<String, Recipe>;
    type Extra = ();

    fn parse(s: &str, _state: &ProblemState<Self::Extra>) -> Self::Input {
        s.split('\n')
            .map(|row| {
                let reaction: Vec<&str> = row.split("=>").collect();

                let result = Ingredient::new(reaction[1]);
                let ingredients = reaction[0].split(',').map(Ingredient::new).collect();

                (
                    result.material.clone(),
                    Recipe {
                        result,
                        ingredients,
                    },
                )
            })
            .collect()
    }

    fn part_1(reactions: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        let required_ore = find_ore_for_one_fuel(reactions);

        Some(format!("{}", required_ore))
    }

    fn part_2(reactions: &Self::Input, _state: &ProblemState<Self::Extra>) -> Option<String> {
        // initially estimate our fuel requirements
        let ore_for_one_fuel = find_ore_for_one_fuel(reactions);

        let mut fuel = 0;

        let mut required_ore = 0;
        while required_ore < ONE_TRILLION {
            let fuel_increment = ((ONE_TRILLION - required_ore) / ore_for_one_fuel).max(1);

            fuel += fuel_increment;

            let mut surpluses = HashMap::new();
            required_ore = find_ore(
                &Ingredient {
                    material: "FUEL".into(),
                    cost: fuel,
                },
                &reactions,
                &mut surpluses,
            );
        }

        // we always go over with our last fuel estimate, so subtract 1 from our answer
        Some(format!("{}", fuel - 1))
    }

    fn problem_number() -> usize {
        14
    }
}

fn find_ore_for_one_fuel(reactions: &HashMap<String, Recipe>) -> usize {
    let mut surpluses = HashMap::new();
    find_ore(
        &Ingredient {
            material: "FUEL".into(),
            cost: 1,
        },
        &reactions,
        &mut surpluses,
    )
}

fn find_ore(
    required: &Ingredient,
    reactions: &HashMap<String, Recipe>,
    surpluses: &mut HashMap<String, usize>,
) -> usize {
    log::trace!("{:?}", required);
    if &required.material == "ORE" {
        required.cost
    } else {
        let recipe: &Recipe = reactions
            .get(&required.material)
            .expect("Invalid reaction definition");

        let mut required_amount = required.cost;

        // check if we can offset our cost with some surplus
        if let Some(surplus) = surpluses.get_mut(&recipe.result.material) {
            if *surplus > required_amount {
                *surplus -= required_amount;
                required_amount = 0;
            } else {
                required_amount -= *surplus;
                *surplus = 0;
            }
        }

        if required_amount > 0 {
            let mut cost_multiple = required_amount / recipe.result.cost;
            let remainder = required_amount % recipe.result.cost;

            // if we have a remainder, that means we still need more from the reaction
            if remainder != 0 {
                let needed = remainder;
                cost_multiple += 1;
                let surplus = recipe.result.cost - needed;

                if let Some(existing) = surpluses.get_mut(&recipe.result.material) {
                    *existing += surplus;
                } else {
                    surpluses.insert(recipe.result.material.clone(), surplus);
                }
            }

            recipe
                .ingredients
                .iter()
                .map(|ingredient| {
                    let mut updated_ingredient = ingredient.clone();
                    updated_ingredient.cost *= cost_multiple;

                    find_ore(&updated_ingredient, reactions, surpluses)
                })
                .sum()
        } else {
            0
        }
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    example!(Fourteen;
    RunFor::Part1, (), r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#,
        RunFor::Both, (), r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#,
    RunFor::Both, (), r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"#,
    RunFor::Both, (), r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"#
        );
    run::<Fourteen>((), include_str!("14_input.txt"));
}

#[cfg(test)]
mod fourteen {
    use super::*;
    use advent_of_code_2019::problem::assert_solution;

    #[test]
    fn test() {
        assert_solution::<Fourteen>(include_str!("14_input.txt"), (), "365768", "3756877");
    }
}
