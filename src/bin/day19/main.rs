use advent_of_code::{create_runner, named, Named, Runner};
use lazy_regex::regex_captures;
use std::str::Lines;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
const NUM_RESOURCE_TYPES: usize = 4;
const RESOURCE_TYPES: [Resource; NUM_RESOURCE_TYPES] = [
    Resource::Ore,
    Resource::Clay,
    Resource::Obsidian,
    Resource::Geode,
];

impl Resource {
    fn from_usize(p: usize) -> Option<Self> {
        RESOURCE_TYPES.iter().copied().find(|&r| r as usize == p)
    }
}

impl<T> std::ops::Index<Resource> for [T; NUM_RESOURCE_TYPES] {
    type Output = T;

    fn index(&self, index: Resource) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Resource> for [T; NUM_RESOURCE_TYPES] {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Resources {
    amounts: [usize; NUM_RESOURCE_TYPES],
}

impl Resources {
    fn empty() -> Self {
        Self {
            amounts: [0usize; NUM_RESOURCE_TYPES],
        }
    }
}

impl std::ops::Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        let mut amounts = self.amounts;
        for (a, b) in amounts.iter_mut().zip(rhs.amounts.into_iter()) {
            *a += b
        }
        Self::Output { amounts }
    }
}

impl std::ops::Add<Resource> for Resources {
    type Output = Resources;

    fn add(self, rhs: Resource) -> Self::Output {
        let mut amounts = self.amounts;
        amounts[rhs] += 1;
        Self::Output { amounts }
    }
}

impl std::ops::Mul<usize> for Resources {
    type Output = Resources;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut amounts = self.amounts;
        for a in amounts.iter_mut() {
            *a *= rhs
        }
        Self::Output { amounts }
    }
}

impl std::ops::Mul<usize> for Resource {
    type Output = Resources;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut amounts = [0usize; NUM_RESOURCE_TYPES];
        amounts[self] = rhs;
        Self::Output { amounts }
    }
}

impl std::ops::Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut amounts = self.amounts;
        for (a, b) in amounts.iter_mut().zip(rhs.amounts.into_iter()) {
            *a -= b
        }
        Self::Output { amounts }
    }
}

impl std::fmt::Debug for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let amounts = self.amounts.iter().enumerate().filter_map(|(i, &a)| {
            if a > 0 {
                Some((a, Resource::from_usize(i).unwrap()))
            } else {
                None
            }
        });
        f.debug_list().entries(amounts).finish()
    }
}

#[derive(PartialEq, Debug)]
struct RobotCost {
    collects: Resource,
    cost: Resources,
}

#[derive(PartialEq, Debug)]
struct State {
    resources: Resources,
    robots: Resources,
    minutes_remaining: usize,
}

impl State {
    fn new(minutes_remaining: usize) -> Self {
        Self {
            resources: Resources::empty(),
            robots: Resource::Ore * 1,
            minutes_remaining,
        }
    }
    fn build_time(&self, robot_cost: &Resources) -> Option<usize> {
        robot_cost
            .amounts
            .iter()
            .zip(self.resources.amounts.iter())
            .zip(self.robots.amounts.iter())
            .map(|((&cost, &have), &robots)| {
                if have >= cost {
                    0
                } else if robots == 0 {
                    usize::MAX
                } else {
                    (cost - have + robots - 1) / robots
                }
            })
            .max()
            .filter(|&t| t < usize::MAX)
    }
    fn is_robot_useful(&self, robot_type: Resource, blueprint: &Blueprint) -> bool {
        robot_type == Resource::Geode
            || blueprint
                .robot_costs
                .iter()
                .any(|c| c.cost.amounts[robot_type] > self.robots.amounts[robot_type])
    }
    fn build_robot(&self, robot: &RobotCost) -> Option<Self> {
        self.build_time(&robot.cost)
            .filter(|&t| t < self.minutes_remaining)
            .map(|t| Self {
                resources: self.resources + self.robots * t - robot.cost + self.robots,
                robots: self.robots + robot.collects,
                minutes_remaining: self.minutes_remaining - t - 1,
            })
    }
    fn collect_resources(&self) -> Self {
        Self {
            resources: self.resources + self.robots * self.minutes_remaining,
            robots: self.robots,
            minutes_remaining: 0,
        }
    }
    fn max_possible_geodes(&self) -> usize {
        // already harvested
        self.resources.amounts[Resource::Geode]
            // each existing robot makes one geode per turn
            + self.robots.amounts[Resource::Geode] * self.minutes_remaining
            // geode robot built every turn mines for remaining turns
            + (self.minutes_remaining * (self.minutes_remaining - 1)) / 2
    }
}

#[derive(PartialEq, Debug)]
struct Blueprint {
    id: usize,
    robot_costs: Vec<RobotCost>,
}

impl Blueprint {
    fn parse(line: &str) -> Option<Self> {
        let (_, id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = regex_captures!(
            r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$",
            line
        )?;
        let robot_costs = vec![
            RobotCost {
                collects: Resource::Ore,
                cost: Resource::Ore * ore_ore.parse().ok()?,
            },
            RobotCost {
                collects: Resource::Clay,
                cost: Resource::Ore * clay_ore.parse().ok()?,
            },
            RobotCost {
                collects: Resource::Obsidian,
                cost: Resource::Ore * obsidian_ore.parse().ok()?
                    + Resource::Clay * obsidian_clay.parse().ok()?,
            },
            RobotCost {
                collects: Resource::Geode,
                cost: Resource::Ore * geode_ore.parse().ok()?
                    + Resource::Obsidian * geode_obsidian.parse().ok()?,
            },
        ];
        Some(Self {
            id: id.parse().ok()?,
            robot_costs,
        })
    }
    fn quality_level(&self) -> usize {
        self.id * self.geodes_opened(24)
    }
    fn geodes_opened(&self, minutes: usize) -> usize {
        let mut max_geodes = 0usize;
        self.geodes_opened_rec(State::new(minutes), &mut max_geodes)
    }
    fn geodes_opened_rec(&self, state: State, max_geodes: &mut usize) -> usize {
        if state.minutes_remaining == 0 {
            let num_geodes = state.resources.amounts[Resource::Geode];
            *max_geodes = num_geodes.max(*max_geodes);
            num_geodes
        } else if state.max_possible_geodes() < *max_geodes {
            0
        } else {
            self.robot_costs
                .iter()
                .filter(|robot| state.is_robot_useful(robot.collects, self))
                .filter_map(|robot| state.build_robot(robot))
                .map(|s| self.geodes_opened_rec(s, max_geodes))
                .max()
                .unwrap_or_else(|| self.geodes_opened_rec(state.collect_resources(), max_geodes))
        }
    }
}

fn part1(input: Lines) -> String {
    input
        .flat_map(Blueprint::parse)
        .map(|b| b.quality_level())
        .sum::<usize>()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .flat_map(Blueprint::parse)
        .take(3)
        .map(|b| b.geodes_opened(32))
        .product::<usize>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let runner: &Runner = create_runner!();
    runner.run(named!(part1), input);
    runner.run(named!(part2), input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::verify;

    #[test]
    fn parse_blueprint() {
        let blueprints: Vec<Blueprint> = include_str!("example.txt")
            .lines()
            .flat_map(Blueprint::parse)
            .collect();
        assert_eq!(blueprints.len(), 2);
        let expected = Blueprint {
            id: 1,
            robot_costs: vec![
                RobotCost {
                    collects: Resource::Ore,
                    cost: Resource::Ore * 4,
                },
                RobotCost {
                    collects: Resource::Clay,
                    cost: Resource::Ore * 2,
                },
                RobotCost {
                    collects: Resource::Obsidian,
                    cost: Resource::Ore * 3 + Resource::Clay * 14,
                },
                RobotCost {
                    collects: Resource::Geode,
                    cost: Resource::Ore * 2 + Resource::Obsidian * 7,
                },
            ],
        };
        assert_eq!(blueprints.first(), Some(&expected));
    }

    #[test]
    fn resource_math() {
        let one_ore = Resources::empty() + Resource::Ore;
        assert_eq!(one_ore.amounts[Resource::Ore], 1);

        let one_clay = Resources::empty() + Resource::Clay;
        assert_eq!(one_clay.amounts[Resource::Clay], 1);

        let one_both = one_ore + one_clay;
        assert_eq!(one_both.amounts[Resource::Ore], 1);
        assert_eq!(one_both.amounts[Resource::Clay], 1);

        let two_ore = one_ore + one_ore;
        assert_eq!(two_ore.amounts[Resource::Ore], 2);
        assert_eq!(one_ore.amounts[Resource::Ore], 1);

        let three_ore = two_ore + Resource::Ore;
        assert_eq!(three_ore.amounts[Resource::Ore], 3);
        assert_eq!(two_ore.amounts[Resource::Ore], 2);
    }

    #[test]
    fn build_time() {
        // can build now
        assert_eq!(
            State {
                resources: Resource::Ore * 5,
                robots: Resource::Ore * 0,
                minutes_remaining: 10,
            }
            .build_time(&(Resource::Ore * 5)),
            Some(0)
        );

        // can build soon
        assert_eq!(
            State {
                resources: Resource::Ore * 0,
                robots: Resource::Ore * 1,
                minutes_remaining: 10,
            }
            .build_time(&(Resource::Ore * 5)),
            Some(5)
        );

        // can build after time expires
        assert_eq!(
            State {
                resources: Resource::Ore * 0,
                robots: Resource::Ore * 1,
                minutes_remaining: 2,
            }
            .build_time(&(Resource::Ore * 5)),
            Some(5)
        );

        // can never build
        assert_eq!(
            State {
                resources: Resource::Ore * 0,
                robots: Resource::Ore * 0,
                minutes_remaining: 10,
            }
            .build_time(&(Resource::Ore * 5)),
            None
        );

        // picks longer of 2 resources
        assert_eq!(
            State {
                resources: Resource::Ore * 0 + Resource::Clay * 5,
                robots: Resource::Ore * 3 + Resource::Clay * 1,
                minutes_remaining: 10,
            }
            .build_time(&(Resource::Ore * 4 + Resource::Clay * 6)),
            Some(2)
        );
    }

    #[test]
    fn is_robot_useful() {
        let blueprint = Blueprint {
            id: 1,
            robot_costs: vec![
                RobotCost {
                    collects: Resource::Ore,
                    cost: Resource::Ore * 1,
                },
                RobotCost {
                    collects: Resource::Clay,
                    cost: Resource::Ore * 3,
                },
            ],
        };
        assert!(State {
            resources: Resource::Ore * 0,
            robots: Resource::Ore * 1,
            minutes_remaining: 10,
        }
        .is_robot_useful(Resource::Ore, &blueprint));
        assert!(!State {
            resources: Resource::Ore * 3,
            robots: Resource::Ore * 3,
            minutes_remaining: 10,
        }
        .is_robot_useful(Resource::Ore, &blueprint));

        // Geode robots are always useful
        assert!(State {
            resources: Resource::Ore * 0,
            robots: Resource::Geode * 10000,
            minutes_remaining: 10,
        }
        .is_robot_useful(Resource::Geode, &blueprint));
    }

    #[test]
    fn next_state() {
        let one_ore = Resources::empty() + Resource::Ore;
        let one_clay = Resources::empty() + Resource::Clay;
        let clay_robot = RobotCost {
            collects: Resource::Clay,
            cost: one_ore,
        };

        let state = State::new(5);
        assert_eq!(state.minutes_remaining, 5);
        assert_eq!(state.robots, one_ore);
        assert_eq!(state.resources, Resources::empty());

        let state = state.build_robot(&clay_robot).unwrap();
        assert_eq!(state.minutes_remaining, 3);
        assert_eq!(state.robots, one_ore + one_clay);
        assert_eq!(state.resources, one_ore);

        let state = state.collect_resources();
        assert_eq!(state.minutes_remaining, 0);
        assert_eq!(state.robots, one_ore + one_clay);
        assert_eq!(state.resources, Resource::Ore * 4 + Resource::Clay * 3);
    }

    #[test]
    fn geodes_opened_part1() {
        let blueprints: Vec<Blueprint> = include_str!("example.txt")
            .lines()
            .flat_map(Blueprint::parse)
            .collect();
        assert_eq!(blueprints[0].geodes_opened(24), 9);
        assert_eq!(blueprints[1].geodes_opened(24), 12);
    }

    #[test]
    fn geodes_opened_part2() {
        let blueprints: Vec<Blueprint> = include_str!("example.txt")
            .lines()
            .flat_map(Blueprint::parse)
            .collect();
        assert_eq!(blueprints[0].geodes_opened(32), 56);
        assert_eq!(blueprints[1].geodes_opened(32), 62);
    }

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "33");
        verify!(part2, input, "3472");
    }
}
