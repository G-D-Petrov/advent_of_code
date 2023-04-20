enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Outcome {
    fn from_str(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome character"),
        }
    }
}

impl RPS {
    fn from_str(input: &str) -> RPS {
        match input {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("Invalid RPS character"),
        }
    }
}

struct Position {
    side_a: RPS,
    side_b: RPS,
}

impl Position {
    fn new(input: &str) -> Position {
        let mut parts = input.split(" ");
        let side_a = RPS::from_str(parts.next().unwrap());
        let side_b = RPS::from_str(parts.next().unwrap());
    
        Position { side_a, side_b }
    }

    fn simulate(&self) -> usize {
        match self.side_b {
            RPS::Rock => {
                let outcome = match self.side_a {
                    RPS::Rock => 3,
                    RPS::Paper => 0,
                    RPS::Scissors => 6,
                };

                outcome + RPS::Rock as usize
            },
            RPS::Paper => {
                let outcome = match self.side_a {
                    RPS::Rock => 6,
                    RPS::Paper => 3,
                    RPS::Scissors => 0,
                };

                outcome + RPS::Paper as usize
            },
            RPS::Scissors => {
                let outcome = match self.side_a {
                    RPS::Rock => 0,
                    RPS::Paper => 6,
                    RPS::Scissors => 3,
                };

                outcome + RPS::Scissors as usize
            },
        }
    }
}

struct Strategy {
    side_a: RPS,
    outcome: Outcome,
}

impl Strategy {
    fn new(input: &str) -> Strategy {
        let mut parts = input.split(" ");
        let side_a = RPS::from_str(parts.next().unwrap());
        let outcome = Outcome::from_str(parts.next().unwrap());
    
        Strategy { side_a, outcome }
    }

    fn simulate(&self) -> usize {
        match self.outcome {
            Outcome::Win => {
                let value_to_pick = match self.side_a {
                    RPS::Rock => RPS::Paper,
                    RPS::Paper => RPS::Scissors,
                    RPS::Scissors => RPS::Rock,
                } as usize;

                value_to_pick + Outcome::Win as usize
            },
            Outcome::Draw => {
                let value_to_pick = match self.side_a {
                    RPS::Rock => RPS::Rock,
                    RPS::Paper => RPS::Paper,
                    RPS::Scissors => RPS::Scissors,
                } as usize;

                value_to_pick + Outcome::Draw as usize
            },
            Outcome::Loss => {
                let value_to_pick = match self.side_a {
                    RPS::Rock => RPS::Scissors,
                    RPS::Paper => RPS::Rock,
                    RPS::Scissors => RPS::Paper,
                } as usize;

                value_to_pick + Outcome::Loss as usize
            },
        }
    }
}

pub enum StrategyType {
    Position,
    Strategy,
}

pub fn calculate_strategy_value(input: &str, case: StrategyType) -> usize {
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        let value = match case {
            StrategyType::Position => {
                let pos = Position::new(line);
                pos.simulate()
            },
            StrategyType::Strategy => {
                let strat = Strategy::new(line);
                strat.simulate()
            },
        };
        total += value;
    }
    
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_1() {
        let input = include_str!("test_input.txt");
        let result = calculate_strategy_value(input, StrategyType::Position);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_1_2() {
        let input = include_str!("test_1_2.txt");
        let result = calculate_strategy_value(input, StrategyType::Position);
        assert_eq!(result, 10624);
    }

    #[test]
    fn test_part_2_1() {
        let input = include_str!("test_input.txt");
        let result = calculate_strategy_value(input, StrategyType::Strategy);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_2_2() {
        let input = include_str!("test_2_2.txt");
        let result = calculate_strategy_value(input, StrategyType::Strategy);
        assert_eq!(result, 14060);
    }
}
