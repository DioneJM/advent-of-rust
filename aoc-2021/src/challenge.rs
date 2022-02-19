
pub trait ChallengeSolver {
    fn solve_part_one(&self) -> String;
    fn solve_part_two(&self) -> String;
    fn solve_all(&self) -> (String, String) {
        (self.solve_part_one(), self.solve_part_two())
    }
}