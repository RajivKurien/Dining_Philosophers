use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;
use crate::dining_philosophers::philosophers::{Philosopher, State};
use crate::dining_philosophers::table::{Table, SeatingPosition};
use crate::dining_philosophers::philosophers::State::LeftThinking;

#[derive(Debug, PartialEq)]
pub struct LeftThinkingPhilosopher {
    left_fork: Option<Fork>,
    seating_position: SeatingPosition,
}

impl LeftThinkingPhilosopher {
    pub fn new(left_fork: Fork, seating_position: SeatingPosition) -> LeftThinkingPhilosopher {
        LeftThinkingPhilosopher {
            left_fork: Some(left_fork),
            seating_position,
        }
    }
    fn take_right(&mut self, fork: Fork) -> EatingPhilosopher {
        EatingPhilosopher::new(self.left_fork.take().unwrap(), fork, self.seating_position)
    }
    fn drop_left(&mut self) -> (ThinkingPhilosopher, Fork) {
        (ThinkingPhilosopher::new(self.seating_position), self.left_fork.take().unwrap())
    }
}

impl Philosopher for LeftThinkingPhilosopher {
    fn act(&mut self, table: &mut Table) -> Box<Philosopher + Send> {
        match self.seating_position.get_right_fork(table) {
            None => {
                println!("{}: Not right, back to thinking", self.seating_position.position);
                let (philosopher, fork) = self.drop_left();
                self.seating_position.return_left_fork(fork, table);
                Box::new(philosopher)
            }
            Some(fork) => {
                println!("{}: Got right. Eating!", self.seating_position.position);
                Box::new(self.take_right(fork))
            }
        }
    }

    fn state(&self) -> State {
        LeftThinking
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
    use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;
    use crate::dining_philosophers::table::{SeatingPosition, Table};
    use crate::dining_philosophers::philosophers::State::{LeftThinking, RightThinking, Thinking, Eating};
    use crate::dining_philosophers::philosophers::Philosopher;

    #[test]
    fn take_right_becomes_eating() {
        let seating_position = SeatingPosition { position: 0 };

        let mut unit = LeftThinkingPhilosopher { left_fork: Some(Fork), seating_position };

        assert_eq!(unit.take_right(Fork), EatingPhilosopher::new(Fork, Fork, seating_position));
    }

    #[test]
    fn drop_left_becomes_thinking() {
        let seating_position = SeatingPosition { position: 0 };
        let mut unit = LeftThinkingPhilosopher { left_fork: Some(Fork), seating_position };

        let (unit, _fork) = unit.drop_left();

        assert_eq!(unit, ThinkingPhilosopher::new(seating_position));
    }

    #[test]
    fn state_is_left_thinking() {
        let seating_position = SeatingPosition { position: 0 };
        let unit = LeftThinkingPhilosopher::new(Fork, seating_position);

        assert_eq!(unit.state(), LeftThinking);
    }

    #[test]
    fn changes_to_eating_when_right_fork_available() {
        let mut table = Table::new(2);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let fork = seating_position.get_left_fork(&mut table).unwrap();
        let mut unit: Box<Philosopher> = Box::new(LeftThinkingPhilosopher::new(fork, seating_position));

        unit = unit.act(&mut table);

        assert_eq!(unit.state(), Eating);
    }

    #[test]
    fn changes_to_thinking_when_right_fork_is_not_available() {
        let mut table = Table::new(1);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let mut fork = seating_position.get_left_fork(&mut table);
        let mut unit: Box<Philosopher> = Box::new(LeftThinkingPhilosopher::new(fork.take().unwrap(), seating_position));

        unit = unit.act(&mut table);

        assert_eq!(unit.state(), Thinking);
    }

    #[test]
    fn returns_left_fork_when_right_fork_is_not_available() {
        let mut table = Table::new(1);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let mut fork = seating_position.get_left_fork(&mut table);
        let mut unit: Box<Philosopher> = Box::new(LeftThinkingPhilosopher::new(fork.take().unwrap(), seating_position));

        unit = unit.act(&mut table);

        let seating_position = table.get_seating_positions().pop().unwrap();

        assert_eq!(seating_position.get_left_fork(&mut table), Some(Fork));
    }
}
