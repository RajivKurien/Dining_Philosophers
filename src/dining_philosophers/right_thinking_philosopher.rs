use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;
use crate::dining_philosophers::table::{SeatingPosition, Table};
use crate::dining_philosophers::philosophers::{Philosopher, State};
use crate::dining_philosophers::philosophers::State::RightThinking;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct RightThinkingPhilosopher {
    right_fork: Option<Fork>,
    seating_position: Arc<SeatingPosition>,
}

impl RightThinkingPhilosopher {
    pub fn new(right_fork: Fork, seating_position: Arc<SeatingPosition>) -> RightThinkingPhilosopher {
        RightThinkingPhilosopher { right_fork: Some(right_fork), seating_position }
    }
    fn take_left(&mut self, fork: Fork) -> EatingPhilosopher {
        EatingPhilosopher::new(fork, self.right_fork.take().unwrap(), self.seating_position.clone())
    }
    fn drop_right(&mut self) -> (ThinkingPhilosopher, Fork) {
        (ThinkingPhilosopher::new(self.seating_position.clone()), self.right_fork.take().unwrap())
    }
}

impl Philosopher for RightThinkingPhilosopher {
    fn act(&mut self, table: &mut Table) -> Box<Philosopher + Send+ Sync> {
        match self.seating_position.get_left_fork(table) {
            None => {
                println!("{}: Not left, back to thinking", self.seating_position.position);
                let (philosopher, fork) = self.drop_right();
                self.seating_position.return_right_fork(fork, table);
                Box::new(philosopher)
            }
            Some(fork) => {
                println!("{}: Got left. Eating!", self.seating_position.position);
                Box::new(self.take_left(fork))
            }
        }
    }

    fn state(&self) -> State {
        RightThinking
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
    use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;
    use crate::dining_philosophers::table::{SeatingPosition, Table};
    use crate::dining_philosophers::philosophers::State::{RightThinking, Eating, Thinking};
    use crate::dining_philosophers::philosophers::Philosopher;
    use std::sync::Arc;

    #[test]
    fn right_thinking_take_left_becomes_eating() {
        let seating_position = Arc::new(SeatingPosition { position: 0 });
        let mut unit = RightThinkingPhilosopher { right_fork: Some(Fork), seating_position: seating_position.clone() };

        assert_eq!(unit.take_left(Fork), EatingPhilosopher::new(Fork, Fork, seating_position));
    }

    #[test]
    fn right_thinking_drop_right_becomes_thinking() {
        let seating_position = Arc::new(SeatingPosition { position: 0 });
        let mut unit = RightThinkingPhilosopher { right_fork: Some(Fork), seating_position: seating_position.clone() };

        let (unit, _fork) = unit.drop_right();

        assert_eq!(unit, ThinkingPhilosopher::new(seating_position));
    }

    #[test]
    fn state_is_right_thinking() {
        let seating_position = Arc::new(SeatingPosition { position: 0 });
        let unit = RightThinkingPhilosopher::new(Fork, seating_position);

        assert_eq!(unit.state(), RightThinking);
    }

    #[test]
    fn changes_to_eating_when_left_fork_available() {
        let mut table = Table::new(2);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let fork = seating_position.get_right_fork(&mut table).unwrap();
        let mut unit: Box<Philosopher> = Box::new(RightThinkingPhilosopher::new(fork, Arc::new(seating_position)));

        unit = unit.act(&mut table);

        assert_eq!(unit.state(), Eating);
    }

    #[test]
    fn changes_to_thinking_when_left_fork_is_not_available() {
        let mut table = Table::new(1);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let mut fork = seating_position.get_right_fork(&mut table);
        let mut unit: Box<Philosopher> = Box::new(RightThinkingPhilosopher::new(fork.take().unwrap(), Arc::new(seating_position)));

        unit = unit.act(&mut table);

        assert_eq!(unit.state(), Thinking);
    }

    #[test]
    fn returns_right_fork_when_left_fork_is_not_available() {
        let mut table = Table::new(1);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let mut fork = seating_position.get_right_fork(&mut table);
        let mut unit: Box<Philosopher> = Box::new(RightThinkingPhilosopher::new(fork.take().unwrap(), Arc::new(seating_position)));

        unit = unit.act(&mut table);

        let seating_position = table.get_seating_positions().pop().unwrap();

        assert_eq!(seating_position.get_right_fork(&mut table), Some(Fork));
    }
}
