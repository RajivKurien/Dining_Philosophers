use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;
use crate::dining_philosophers::table::{SeatingPosition, Table};
use crate::dining_philosophers::philosophers::{Philosopher, State};
use crate::dining_philosophers::philosophers::State::Eating;

#[derive(Debug, PartialEq)]
pub struct EatingPhilosopher {
    left_fork: Option<Fork>,
    right_fork: Option<Fork>,
    seating_position: SeatingPosition,
}

impl EatingPhilosopher {
    pub fn new(left_fork: Fork, right_fork: Fork, seating_position: SeatingPosition) -> EatingPhilosopher {
        EatingPhilosopher {
            left_fork: Some(left_fork),
            right_fork: Some(right_fork),
            seating_position,
        }
    }

    fn drop_left(&mut self) -> (RightThinkingPhilosopher, Fork) {
        (RightThinkingPhilosopher::new(self.right_fork.take().unwrap(), self.seating_position), self.left_fork.take().unwrap())
    }
    fn drop_right(&mut self) -> (LeftThinkingPhilosopher, Fork) {
        (LeftThinkingPhilosopher::new(self.left_fork.take().unwrap(), self.seating_position), self.right_fork.take().unwrap())
    }
}

impl Philosopher for EatingPhilosopher {
    fn act(&mut self, table: &mut Table) -> Box<Philosopher + Send> {
        println!("{}: Drop left, to right thinking", self.seating_position.position);
        let (philosopher, fork) = self.drop_left();
        self.seating_position.return_left_fork(fork, table);
        Box::new(philosopher)
    }

    fn state(&self) -> State {
        Eating
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
    use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;
    use crate::dining_philosophers::table::{SeatingPosition, Table};
    use crate::dining_philosophers::philosophers::State::{Eating, RightThinking};
    use crate::dining_philosophers::philosophers::Philosopher;

    #[test]
    fn eating_drop_right_becomes_left_thinking() {
        let seating_position = SeatingPosition { position: 0 };
        let mut unit = EatingPhilosopher { left_fork: Some(Fork), right_fork: Some(Fork), seating_position };

        let (unit, _fork) = unit.drop_right();

        assert_eq!(unit, LeftThinkingPhilosopher::new(Fork, seating_position));
    }

    #[test]
    fn eating_drop_left_becomes_right_thinking() {
        let seating_position = SeatingPosition { position: 0 };
        let mut unit = EatingPhilosopher { left_fork: Some(Fork), right_fork: Some(Fork), seating_position };

        let (unit, _fork) = unit.drop_left();

        assert_eq!(unit, RightThinkingPhilosopher::new(Fork, seating_position));
    }

    #[test]
    fn state_is_eating() {
        let seating_position = SeatingPosition { position: 0 };
        let unit = EatingPhilosopher::new(Fork, Fork, seating_position);

        assert_eq!(unit.state(), Eating);
    }

    #[test]
    fn changes_to_right_thinking() {
        let mut table = Table::new(2);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let left_fork = seating_position.get_left_fork(&mut table).unwrap();
        let right_fork = seating_position.get_right_fork(&mut table).unwrap();
        let mut unit: Box<Philosopher> = Box::new(EatingPhilosopher::new(left_fork, right_fork, seating_position));

        unit = unit.act(&mut table);

        assert_eq!(unit.state(), RightThinking);
    }

    #[test]
    fn acts_to_return_left_fork() {
        let mut table = Table::new(2);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let left_fork = seating_position.get_left_fork(&mut table).unwrap();
        let right_fork = seating_position.get_right_fork(&mut table).unwrap();
        let mut unit: Box<Philosopher> = Box::new(EatingPhilosopher::new(left_fork, right_fork, seating_position));

        unit = unit.act(&mut table);

        let seating_position = table.get_seating_positions().pop().unwrap();

        assert_eq!(seating_position.get_left_fork(&mut table), Some(Fork));
    }
}
