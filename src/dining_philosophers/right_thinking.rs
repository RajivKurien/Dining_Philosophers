use std::sync::Arc;

use crate::dining_philosophers::eating::Eating;
use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::philosopher::{StateMachine, State};
use crate::dining_philosophers::table::TableInteraction;
use crate::dining_philosophers::thinking::Thinking;

#[derive(Debug, PartialEq)]
pub struct RightThinking {
    right_fork: Option<Fork>,
    pub seating_position: Arc<TableInteraction>,
}

impl RightThinking {
    pub fn new(right_fork: Fork, seating_position: Arc<TableInteraction>) -> RightThinking {
        RightThinking { right_fork: Some(right_fork), seating_position }
    }
    fn take_left(&mut self, fork: Fork) -> Eating {
        Eating::new(fork, self.right_fork.take().unwrap(), self.seating_position.clone())
    }
    fn drop_right(&mut self) -> (Thinking, Fork) {
        (Thinking::new(self.seating_position.clone()), self.right_fork.take().unwrap())
    }
}

impl StateMachine for RightThinking {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        match self.seating_position.get_left_fork() {
            None => {
                println!("{}: Not left, back to thinking", self.seating_position.position);
                let (philosopher, fork) = self.drop_right();
                self.seating_position.return_right_fork(fork);
                Box::new(philosopher)
            }
            Some(fork) => {
                println!("{}: Got left. Eating!", self.seating_position.position);
                Box::new(self.take_left(fork))
            }
        }
    }

    fn state(&self) -> State {
        State::RightThinking
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::eating::Eating;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::philosopher::{StateMachine, State};
    use crate::dining_philosophers::right_thinking::RightThinking;
    use crate::dining_philosophers::table::{Table, TableInteraction};
    use crate::dining_philosophers::thinking::Thinking;

    #[test]
    fn right_thinking_take_left_becomes_eating() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let mut unit = RightThinking { right_fork: Some(Fork), seating_position: seating_position.clone() };

        assert_eq!(unit.take_left(Fork), Eating::new(Fork, Fork, seating_position));
    }

    #[test]
    fn right_thinking_drop_right_becomes_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let mut unit = RightThinking { right_fork: Some(Fork), seating_position: seating_position.clone() };

        let (unit, _fork) = unit.drop_right();

        assert_eq!(unit, Thinking::new(seating_position));
    }

    #[test]
    fn state_is_right_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let unit = RightThinking::new(Fork, seating_position);

        assert_eq!(unit.state(), State::RightThinking);
    }

    #[test]
    fn changes_to_eating_when_left_fork_available() {
        let table = Table::new(2);
        let seating_position = table.get_interactions().pop().unwrap();
        let fork = seating_position.get_right_fork().unwrap();
        let mut unit: Box<StateMachine> = Box::new(RightThinking::new(fork, Arc::new(seating_position)));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Eating);
    }

    #[test]
    fn changes_to_thinking_when_left_fork_is_not_available() {
        let table = Table::new(1);
        let seating_position = table.get_interactions().pop().unwrap();
        let mut fork = seating_position.get_right_fork();
        let mut unit: Box<StateMachine> = Box::new(RightThinking::new(fork.take().unwrap(), Arc::new(seating_position)));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Thinking);
    }

    #[test]
    fn returns_right_fork_when_left_fork_is_not_available() {
        let table = Table::new(1);
        let seating_position = Arc::new(table.get_interactions().pop().unwrap());
        let mut fork = seating_position.get_right_fork();
        let mut unit: Box<StateMachine> = Box::new(RightThinking::new(fork.take().unwrap(), Arc::clone(&seating_position)));

        unit.transition();

        assert_eq!(seating_position.get_right_fork(), Some(Fork));
    }
}
