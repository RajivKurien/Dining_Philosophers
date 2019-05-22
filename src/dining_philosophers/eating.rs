use std::sync::Arc;

use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking::LeftThinking;
use crate::dining_philosophers::philosopher::{StateMachine, State};
use crate::dining_philosophers::right_thinking::RightThinking;
use crate::dining_philosophers::table::TableInteraction;

#[derive(Debug, PartialEq)]
pub struct Eating {
    left_fork: Option<Fork>,
    right_fork: Option<Fork>,
    seating_position: Arc<TableInteraction>,
}

impl Eating {
    pub fn new(left_fork: Fork, right_fork: Fork, seating_position: Arc<TableInteraction>) -> Eating {
        Eating {
            left_fork: Some(left_fork),
            right_fork: Some(right_fork),
            seating_position,
        }
    }

    fn drop_left(&mut self) -> (RightThinking, Fork) {
        (RightThinking::new(self.right_fork.take().unwrap(), self.seating_position.clone()), self.left_fork.take().unwrap())
    }
    fn drop_right(&mut self) -> (LeftThinking, Fork) {
        (LeftThinking::new(self.left_fork.take().unwrap(), self.seating_position.clone()), self.right_fork.take().unwrap())
    }
}

impl StateMachine for Eating {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        println!("{}: Drop left, to right thinking", self.seating_position.position);
        let (philosopher, fork) = self.drop_left();
        self.seating_position.return_left_fork(fork);
        Box::new(philosopher)
    }

    fn state(&self) -> State {
        State::Eating
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::eating::Eating;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::left_thinking::LeftThinking;
    use crate::dining_philosophers::philosopher::{StateMachine, State};
    use crate::dining_philosophers::right_thinking::RightThinking;
    use crate::dining_philosophers::table::{Table, TableInteraction};

    #[test]
    fn eating_drop_right_becomes_left_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let mut unit = Eating { left_fork: Some(Fork), right_fork: Some(Fork), seating_position: seating_position.clone() };

        let (unit, _fork) = unit.drop_right();

        assert_eq!(unit, LeftThinking::new(Fork, seating_position));
    }

    #[test]
    fn eating_drop_left_becomes_right_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let mut unit = Eating { left_fork: Some(Fork), right_fork: Some(Fork), seating_position: seating_position.clone() };

        let (unit, _fork) = unit.drop_left();

        assert_eq!(unit, RightThinking::new(Fork, seating_position));
    }

    #[test]
    fn state_is_eating() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let unit = Eating::new(Fork, Fork, seating_position);

        assert_eq!(unit.state(), State::Eating);
    }

    #[test]
    fn changes_to_right_thinking() {
        let table = Table::new(2);
        let seating_position = table.get_interactions().pop().unwrap();
        let left_fork = seating_position.get_left_fork().unwrap();
        let right_fork = seating_position.get_right_fork().unwrap();
        let mut unit: Box<StateMachine> = Box::new(Eating::new(left_fork, right_fork, Arc::new(seating_position)));

        unit = unit.transition();

        assert_eq!(unit.state(), State::RightThinking);
    }

    #[test]
    fn acts_to_return_left_fork() {
        let table = Table::new(2);
        let mut interactions = table.get_interactions();
        let seating_position = Arc::new(interactions.pop().unwrap());
        let left_fork = seating_position.get_left_fork().unwrap();
        let right_fork = seating_position.get_right_fork().unwrap();
        let mut unit: Box<StateMachine> = Box::new(Eating::new(left_fork, right_fork, Arc::clone(&seating_position)));

        unit.transition();

        assert_eq!(seating_position.get_left_fork(), Some(Fork));
    }
}
