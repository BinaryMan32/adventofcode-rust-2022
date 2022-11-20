use super::solution::Solution;

pub struct Runner {
    name: &'static str,
}

impl Runner {
    pub fn create(name: &'static str) -> Runner {
        Runner { name }
    }

    pub fn run(&self, solution: &dyn Solution) {
        println!("{} part1: {}", self.name, solution.part1());
        println!("{} part2: {}", self.name, solution.part2());
    }
}
