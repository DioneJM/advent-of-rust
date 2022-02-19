
pub trait ChallengeSolver {
    fn solve_part_one(&self) -> String {
        String::from("idk :(")
    }
    fn solve_part_two(&self) -> String {
        String::from("idk :(")
    }
    fn solve_all(&self) -> (String, String) {
        (self.solve_part_one(), self.solve_part_two())
    }
}