use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;
use crate::dining_philosophers::philosophers::{Philosopher, State};
use crate::dining_philosophers::table::{Table, SeatingPosition};
use crate::dining_philosophers::philosophers::State::Thinking;

#[derive(Debug, PartialEq)]
pub struct ThinkingPhilosopher {
    seating_position: SeatingPosition
}

impl ThinkingPhilosopher {
    pub fn new(seating_position: SeatingPosition) -> ThinkingPhilosopher {
        ThinkingPhilosopher {
            seating_position,
        }
    }
    fn take_left(&self, fork: Fork) -> LeftThinkingPhilosopher<> {
        LeftThinkingPhilosopher::new(fork, self.seating_position)
    }
    fn take_right(&self, fork: Fork) -> RightThinkingPhilosopher<> {
        RightThinkingPhilosopher::new(fork, self.seating_position)
    }
}

impl Philosopher for ThinkingPhilosopher {
    fn act(&mut self, table: &mut Table) -> Box<Philosopher + Send> {
        match self.seating_position.get_left_fork(table) {
            None => {
                println!("{}: Still thinking", self.seating_position.position);
                Box::new(ThinkingPhilosopher::new(self.seating_position))
            }
            Some(fork) => {
                println!("{}: Got the left one!", self.seating_position.position);
                Box::new(LeftThinkingPhilosopher::new(fork, self.seating_position))
            }
        }
    }

    fn state(&self) -> State {
        Thinking
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;
    use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;
    use crate::dining_philosophers::philosophers::Philosopher;
    use crate::dining_philosophers::table::{Table, SeatingPosition};
    use std::rc::Rc;
    use crate::dining_philosophers::philosophers::State::{Thinking, LeftThinking};

    #[test]
    fn take_left_becomes_left_thinking() {
        let seating_position = SeatingPosition { position: 0 };
        let unit = ThinkingPhilosopher::new(seating_position);

        assert_eq!(unit.take_left(Fork), LeftThinkingPhilosopher::new(Fork, seating_position));
    }

    #[test]
    fn take_right_becomes_right_thinking() {
        let seating_position = SeatingPosition { position: 0 };
        let unit = ThinkingPhilosopher::new(seating_position);

        assert_eq!(unit.take_right(Fork), RightThinkingPhilosopher::new(Fork, seating_position));
    }

    #[test]
    fn state_is_thinking() {
        let seating_position = SeatingPosition { position: 0 };
        let unit = ThinkingPhilosopher::new(seating_position);

        assert_eq!(unit.state(), Thinking);
    }

    #[test]
    fn changes_to_left_when_left_fork_available() {
        let mut table = Table::new(1);
        let seating_position = table.get_seating_positions().pop().unwrap();
        let mut unit: Box<Philosopher> = Box::new(ThinkingPhilosopher::new(seating_position));

        unit = unit.act(&mut table);

        assert_eq!(unit.state(), LeftThinking);
    }

    #[test]
    fn changes_to_thinking_when_left_fork_is_not_available() {
        let mut table = Table::new(1);
        let seating_position = table.get_seating_positions().pop().unwrap();
        seating_position.get_left_fork(&mut table);
        let mut unit: Box<Philosopher> = Box::new(ThinkingPhilosopher::new(seating_position));

        unit = unit.act(&mut table);

        assert_eq!(unit.state(), Thinking);
    }
}
