use std::sync::Arc;

use crate::dining_philosophers::eating::Eating;
use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::philosopher::{StateMachine, State};
use crate::dining_philosophers::table::TableInteraction;
use crate::dining_philosophers::thinking::Thinking;

#[derive(Debug, PartialEq)]
pub struct LeftThinking {
    left_fork: Option<Fork>,
    seating_position: Arc<TableInteraction>,
}

impl LeftThinking {
    pub fn new(left_fork: Fork, seating_position: Arc<TableInteraction>) -> LeftThinking {
        LeftThinking {
            left_fork: Some(left_fork),
            seating_position,
        }
    }
    fn take_right(&mut self, fork: Fork) -> Eating {
        Eating::new(self.left_fork.take().unwrap(), fork, self.seating_position.clone())
    }
    fn drop_left(&mut self) -> (Thinking, Fork) {
        (Thinking::new(self.seating_position.clone()), self.left_fork.take().unwrap())
    }
}

impl StateMachine for LeftThinking {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        match self.seating_position.get_right_fork() {
            None => {
                println!("{}: Not right, back to thinking", self.seating_position.position);
                let (philosopher, fork) = self.drop_left();
                self.seating_position.return_left_fork(fork);
                Box::new(philosopher)
            }
            Some(fork) => {
                println!("{}: Got right. Eating!", self.seating_position.position);
                Box::new(self.take_right(fork))
            }
        }
    }

    fn state(&self) -> State {
        State::LeftThinking
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::eating::Eating;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::left_thinking::LeftThinking;
    use crate::dining_philosophers::philosopher::{StateMachine, State};
    use crate::dining_philosophers::table::{Table, TableInteraction};
    use crate::dining_philosophers::thinking::Thinking;

    #[test]
    fn take_right_becomes_eating() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });

        let mut unit = LeftThinking { left_fork: Some(Fork), seating_position: seating_position.clone() };

        assert_eq!(unit.take_right(Fork), Eating::new(Fork, Fork, seating_position));
    }

    #[test]
    fn drop_left_becomes_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let mut unit = LeftThinking { left_fork: Some(Fork), seating_position: seating_position.clone() };

        let (unit, _fork) = unit.drop_left();

        assert_eq!(unit, Thinking::new(seating_position));
    }

    #[test]
    fn state_is_left_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let unit = LeftThinking::new(Fork, seating_position.clone());

        assert_eq!(unit.state(), State::LeftThinking);
    }

    #[test]
    fn changes_to_eating_when_right_fork_available() {
        let table = Table::new(2);
        let seating_position = table.get_interactions().pop().unwrap();
        let fork = seating_position.get_left_fork().unwrap();
        let mut unit: Box<StateMachine> = Box::new(LeftThinking::new(fork, Arc::new(seating_position)));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Eating);
    }

    #[test]
    fn changes_to_thinking_when_right_fork_is_not_available() {
        let table = Table::new(1);
        let seating_position = table.get_interactions().pop().unwrap();
        let mut fork = seating_position.get_left_fork();
        let mut unit: Box<StateMachine> = Box::new(LeftThinking::new(fork.take().unwrap(), Arc::new(seating_position)));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Thinking);
    }

    #[test]
    fn returns_left_fork_when_right_fork_is_not_available() {
        let table = Table::new(1);
        let seating_position = Arc::new(table.get_interactions().pop().unwrap());
        let mut fork = seating_position.get_left_fork();
        let mut unit: Box<StateMachine> = Box::new(LeftThinking::new(fork.take().unwrap(), Arc::clone(&seating_position)));

        unit.transition();

        assert_eq!(seating_position.get_left_fork(), Some(Fork));
    }
}
